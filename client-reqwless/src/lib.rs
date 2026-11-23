//! Bounded async HTTP transport for the Sony BRAVIA sans-I/O client.
//!
//! The caller supplies a stream and enforces the deadline for the complete
//! exchange. The client uses plain HTTP and requires a global allocator for
//! Sony request serialization and response decoding.

#![no_std]

use embedded_io_async::ErrorKind;
use embedded_io_async::Read;
use embedded_io_async::Write;
use reqwless::client::HttpConnection;
use reqwless::headers::ContentType;
use reqwless::request::Request;
use reqwless::request::RequestBuilder;

use sony_bravia_api as api;
use sony_bravia_client as protocol;

const REQUEST_LIMIT: usize = 512;
const HEADER_LIMIT: usize = 1024;
const BODY_LIMIT: usize = 2048;

/// An HTTP client for one display. Requests are made sequentially per client.
#[derive(Debug, Default)]
pub struct Client {
    protocol: protocol::Client,
}

impl Client {
    /// Create a client starting with request ID 1.
    pub fn new() -> Self {
        Self::default()
    }

    /// Send any supported typed command and decode its response.
    ///
    /// The connection future is polled after the command has been encoded.
    /// For an existing stream, pass `core::future::ready(Ok(stream))`.
    /// Supply the TV's HTTP authority as `host`, including the port if needed.
    /// Apply a deadline across this entire future. HTTP sends the key
    /// unencrypted. Each exchange sends one request with `Connection: close`.
    /// It never follows redirects or uses proxies.
    ///
    /// # Errors
    /// Returns an error if preparation, connection, transport, or response decoding
    /// fails. Request bodies are limited to 512 bytes, response headers to 1024
    /// bytes, and response bodies to 2048 bytes.
    pub async fn execute<C: api::SonyBraviaTvCommand, I: Read + Write>(
        &mut self,
        command: C,
        connect: impl Future<Output = Result<I, ErrorKind>>,
        host: &str,
        psk: Option<&str>,
    ) -> Result<C::Response, Error> {
        let pending = self.protocol.prepare(command)?;
        let io = connect.await.map_err(Error::Io)?;
        exchange(io, &pending, host, psk).await
    }
}

/// Error from preparing a command, performing HTTP, or decoding a response.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid HTTP header or request path")]
    InvalidHeader,
    #[error("request body too large")]
    RequestTooLarge,
    #[error("response headers too large")]
    HeaderTooLarge,
    #[error("response body too large")]
    BodyTooLarge,
    #[error("malformed HTTP response")]
    Malformed,
    #[error("truncated HTTP response")]
    Truncated,
    #[error("HTTP transport failed: {0:?}")]
    Io(ErrorKind),
    #[error("TV rejected authentication")]
    Authentication,
    #[error("TV returned HTTP redirect {0}")]
    Redirect(u16),
    #[error("BRAVIA request failed: {0}")]
    Protocol(#[from] protocol::Error),
}

fn header_error(error: reqwless::Error) -> Error {
    match error {
        reqwless::Error::BufferTooSmall => Error::HeaderTooLarge,
        reqwless::Error::ConnectionAborted => Error::Truncated,
        reqwless::Error::Network(error) => Error::Io(error),
        _ => Error::Malformed,
    }
}

async fn exchange<C: api::SonyBraviaTvCommand, I: Read + Write>(
    io: I,
    pending: &protocol::Pending<C>,
    host: &str,
    psk: Option<&str>,
) -> Result<C::Response, Error> {
    if host.is_empty()
        || host.len() > 64
        || !host
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b".:-".contains(&b))
        || pending.path().len() > 64
        || pending.path().bytes().any(|b| b <= b' ' || b >= 0x7f)
        || psk.is_some_and(|s| {
            s.is_empty() || s.len() > 64 || !s.bytes().all(|b| (0x20..0x7f).contains(&b))
        })
    {
        return Err(Error::InvalidHeader);
    }
    if pending.body().len() > REQUEST_LIMIT {
        return Err(Error::RequestTooLarge);
    }
    let headers = [
        ("Connection", "close"),
        ("Accept-Encoding", "identity"),
        ("X-Auth-PSK", psk.unwrap_or("")),
    ];
    let request = Request::post(pending.path())
        .host(host)
        .content_type(ContentType::ApplicationJson)
        .headers(&headers[..if psk.is_some() { 3 } else { 2 }])
        .body(pending.body().as_bytes())
        .build();
    let mut request_buffer = [0; REQUEST_LIMIT];
    let mut header_buffer = [0; HEADER_LIMIT];
    let mut connection = HttpConnection::Plain(io).into_buffered(&mut request_buffer);
    let response = connection
        .send(request, &mut header_buffer)
        .await
        .map_err(header_error)?;
    let status = response.status.0;
    if status == 401 || status == 403 {
        return Err(Error::Authentication);
    }
    if (300..400).contains(&status) {
        return Err(Error::Redirect(status));
    }
    let mut lengths = 0;
    let mut encodings = 0;
    for (name, value) in response.headers() {
        if name.eq_ignore_ascii_case("content-length") {
            lengths += 1;
        }
        if name.eq_ignore_ascii_case("transfer-encoding") {
            encodings += 1;
            if !value.trim_ascii().eq_ignore_ascii_case(b"chunked") {
                return Err(Error::Malformed);
            }
        }
        if name.eq_ignore_ascii_case("content-encoding") && !value.eq_ignore_ascii_case(b"identity")
        {
            return Err(Error::Malformed);
        }
    }
    if lengths > 1 || encodings > 1 || (lengths != 0 && encodings != 0) {
        return Err(Error::Malformed);
    }
    let length = response.content_length;
    if length.is_some_and(|length| length > BODY_LIMIT) {
        return Err(Error::BodyTooLarge);
    }
    let mut reader = response.body().reader();
    let mut body = [0; BODY_LIMIT + 1];
    let mut used = 0;
    loop {
        if length == Some(used) {
            break;
        }
        // For fixed length, never ask the dependency to read beyond remaining
        // bytes. The extra byte detects overflow for chunked/close-delimited data.
        let end = length.unwrap_or(body.len());
        let n = reader
            .read(&mut body[used..end])
            .await
            .map_err(header_error)?;
        if n == 0 {
            if length.is_some_and(|length| used != length) {
                return Err(Error::Truncated);
            }
            break;
        }
        used += n;
        if used > BODY_LIMIT {
            return Err(Error::BodyTooLarge);
        }
    }
    Ok(pending.complete(status, &body[..used])?)
}

#[cfg(test)]
mod tests {
    extern crate std;

    use core::future::ready;

    use std::vec;
    use std::vec::Vec;

    use super::*;

    struct Io {
        input: Vec<u8>,
        position: usize,
        fragment: usize,
        output: Vec<u8>,
    }
    impl embedded_io_async::ErrorType for Io {
        type Error = core::convert::Infallible;
    }
    impl Read for Io {
        async fn read(&mut self, out: &mut [u8]) -> Result<usize, Self::Error> {
            let n = out
                .len()
                .min(self.fragment)
                .min(self.input.len() - self.position);
            out[..n].copy_from_slice(&self.input[self.position..self.position + n]);
            self.position += n;
            Ok(n)
        }
    }
    impl Write for Io {
        async fn write(&mut self, data: &[u8]) -> Result<usize, Self::Error> {
            self.output.extend_from_slice(data);
            Ok(data.len())
        }
        async fn flush(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
    }
    fn run(input: &[u8], fragment: usize) -> Result<api::system::PowerStatus, Error> {
        let mut io = Io {
            input: input.to_vec(),
            position: 0,
            fragment,
            output: Vec::new(),
        };
        embassy_futures::block_on(Client::new().execute(
            api::system::GetPowerStatus,
            ready(Ok(&mut io)),
            "192.168.1.175:80",
            Some("fixture-key"),
        ))
    }
    const BODY: &[u8] = br#"{"id":1,"result":[{"status":"active"}]}"#;
    fn fixed(body: &[u8]) -> Vec<u8> {
        let mut r =
            std::format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n", body.len()).into_bytes();
        r.extend_from_slice(body);
        r
    }
    #[test]
    fn fixed_length_and_close_delimited_decode_full_envelopes() {
        assert_eq!(
            run(&fixed(BODY), 1).unwrap(),
            api::system::PowerStatus::Active
        );
        let mut r = b"HTTP/1.0 200 OK\r\n\r\n".to_vec();
        r.extend_from_slice(BODY);
        assert_eq!(run(&r, 7).unwrap(), api::system::PowerStatus::Active);
    }
    #[test]
    fn fragmented_chunked_and_id_validation() {
        let mut r = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n".to_vec();
        for chunk in BODY.chunks(9) {
            r.extend_from_slice(std::format!("{:x}\r\n", chunk.len()).as_bytes());
            r.extend_from_slice(chunk);
            r.extend_from_slice(b"\r\n");
        }
        r.extend_from_slice(b"0\r\n\r\n");
        assert_eq!(run(&r, 1).unwrap(), api::system::PowerStatus::Active);
        assert!(matches!(
            run(&fixed(br#"{"id":2,"result":[{"status":"active"}]}"#), 3),
            Err(Error::Protocol(protocol::Error::MismatchedId { .. }))
        ));
    }
    #[test]
    fn truncated_and_malformed_framing_are_rejected() {
        assert!(matches!(
            run(b"HTTP/1.1 200 OK\r\nContent-Length: 99\r\n\r\n{}", 1),
            Err(Error::Truncated)
        ));
        assert!(matches!(
            run(
                b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\nContent-Length: 0\r\n\r\n",
                1
            ),
            Err(Error::Malformed)
        ));
        assert!(matches!(
            run(
                b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\nTransfer-Encoding: chunked\r\n\r\n",
                1
            ),
            Err(Error::Malformed)
        ));
        assert!(matches!(
            run(b"HTTP/1.1 200 OK\r\nTransfer-Encoding: gzip\r\n\r\n", 1),
            Err(Error::Malformed)
        ));
    }
    #[test]
    fn header_and_fixed_chunked_close_body_limits_are_enforced() {
        let mut header = b"HTTP/1.1 200 OK\r\nX-Large: ".to_vec();
        header.extend(vec![b'a'; HEADER_LIMIT]);
        header.extend_from_slice(b"\r\n\r\n");
        assert!(matches!(run(&header, 17), Err(Error::HeaderTooLarge)));
        let large = vec![b'a'; BODY_LIMIT + 1];
        assert!(matches!(run(&fixed(&large), 64), Err(Error::BodyTooLarge)));
        let mut r = b"HTTP/1.0 200 OK\r\n\r\n".to_vec();
        r.extend_from_slice(&large);
        assert!(matches!(run(&r, 64), Err(Error::BodyTooLarge)));
        let mut r = std::format!(
            "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n{:x}\r\n",
            large.len()
        )
        .into_bytes();
        r.extend_from_slice(&large);
        r.extend_from_slice(b"\r\n0\r\n\r\n");
        assert!(matches!(run(&r, 7), Err(Error::BodyTooLarge)));
        // Exact limit succeeds, including when chunk decoding must consume its terminator.
        let mut body = BODY.to_vec();
        body.resize(BODY_LIMIT, b' ');
        assert_eq!(
            run(&fixed(&body), 64).unwrap(),
            api::system::PowerStatus::Active
        );
    }
    #[test]
    fn redirects_are_not_followed_and_authentication_is_distinct() {
        assert!(matches!(
            run(b"HTTP/1.1 401 Unauthorized\r\nContent-Length: 0\r\n\r\n", 1),
            Err(Error::Authentication)
        ));
        let mut io = Io { input: b"HTTP/1.1 302 Found\r\nLocation: http://other.example/\r\nContent-Length: 0\r\n\r\n".to_vec(), position: 0, fragment: 1, output: Vec::new() };
        let result = embassy_futures::block_on(Client::new().execute(
            api::system::GetPowerStatus,
            ready(Ok(&mut io)),
            "tv:80",
            Some("fixture-key"),
        ));
        assert!(matches!(result, Err(Error::Redirect(302))));
        let sent = std::str::from_utf8(&io.output).unwrap();
        assert_eq!(sent.matches("POST ").count(), 1);
        assert!(sent.contains("X-Auth-PSK: fixture-key\r\n"));
        assert!(!sent.contains("other.example"));
    }
    #[test]
    fn header_injection_is_rejected_before_any_io() {
        let mut io = Io {
            input: Vec::new(),
            position: 0,
            fragment: 1,
            output: Vec::new(),
        };
        assert!(matches!(
            embassy_futures::block_on(Client::new().execute(
                api::system::GetPowerStatus,
                ready(Ok(&mut io)),
                "tv",
                Some("key\r\nInjected: x")
            )),
            Err(Error::InvalidHeader)
        ));
        assert!(io.output.is_empty());
    }

    #[test]
    fn reports_sony_error_in_http_success() {
        let response = fixed(br#"{"id":1,"error":[401,"Unauthorized"]}"#);
        assert!(matches!(
            run(&response, 1),
            Err(Error::Protocol(protocol::Error::Device(error)))
                if error.code == api::ErrorCode::Unauthorized
        ));
    }

    #[test]
    fn failed_connection_consumes_request_id() {
        let mut client = Client::new();
        assert!(matches!(
            embassy_futures::block_on(client.execute(
                api::system::GetPowerStatus,
                ready(Err::<Io, _>(ErrorKind::ConnectionRefused)),
                "tv",
                None,
            )),
            Err(Error::Io(ErrorKind::ConnectionRefused))
        ));

        let io = Io {
            input: fixed(br#"{"id":2,"result":[{"status":"active"}]}"#),
            position: 0,
            fragment: 1,
            output: Vec::new(),
        };
        assert_eq!(
            embassy_futures::block_on(client.execute(
                api::system::GetPowerStatus,
                ready(Ok(io)),
                "tv",
                None,
            ))
            .unwrap(),
            api::system::PowerStatus::Active
        );
    }
}
