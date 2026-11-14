use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

use sony_bravia_api::audio::SetAudioMute;
use sony_bravia_api::system::GetPowerStatus;
use sony_bravia_api::system::PowerStatus;
use sony_bravia_client::Error as ProtocolError;
use sony_bravia_reqwest::Client;
use sony_bravia_reqwest::Error;

fn mock_server(responses: Vec<(u16, &'static str)>) -> (String, thread::JoinHandle<Vec<String>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let server = thread::spawn(move || {
        let mut requests = Vec::new();
        for (status, body) in responses {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            let mut received = Vec::new();
            let mut buf = [0; 1024];
            loop {
                let count = stream.read(&mut buf).unwrap();
                assert_ne!(count, 0);
                received.extend_from_slice(&buf[..count]);
                if let Some(split) = received.windows(4).position(|part| part == b"\r\n\r\n") {
                    let headers = String::from_utf8_lossy(&received[..split]);
                    let length: usize = headers
                        .lines()
                        .find_map(|line| {
                            let (name, value) = line.split_once(':')?;
                            name.eq_ignore_ascii_case("content-length")
                                .then(|| value.trim().parse().unwrap())
                        })
                        .unwrap();
                    if received.len() >= split + 4 + length {
                        requests.push(String::from_utf8(received).unwrap());
                        break;
                    }
                }
            }
            write!(
                stream,
                "HTTP/1.1 {status} Test\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            )
            .unwrap();
        }
        requests
    });
    (format!("http://{address}"), server)
}

#[tokio::test]
async fn executes_different_commands_and_propagates_http_errors() {
    let (url, server) = mock_server(vec![
        (200, r#"{"id":1,"result":[{"status":"active"}]}"#),
        (200, r#"{"id":2,"result":[0]}"#),
        (401, r#"{"id":3,"error":[401,"Unauthorized"]}"#),
    ]);
    let mut client = Client::new(&url, Some("test-key")).unwrap();
    assert!(!format!("{client:?}").contains("test-key"));
    assert!(matches!(
        client.execute(GetPowerStatus).await.unwrap(),
        PowerStatus::Active
    ));
    client.execute(SetAudioMute { status: true }).await.unwrap();
    assert!(matches!(
        client.execute(GetPowerStatus).await,
        Err(Error::Protocol(ProtocolError::Http {
            status: 401,
            device_error: Some(_),
            ..
        }))
    ));
    let requests = server.join().unwrap();
    for (index, (request, (path, method))) in requests
        .iter()
        .zip([
            ("/sony/system", "getPowerStatus"),
            ("/sony/audio", "setAudioMute"),
            ("/sony/system", "getPowerStatus"),
        ])
        .enumerate()
    {
        let (headers, body) = request.split_once("\r\n\r\n").unwrap();
        assert!(headers.starts_with(&format!("POST {path} HTTP/1.1\r\n")));
        assert!(
            headers
                .to_ascii_lowercase()
                .contains("x-auth-psk: test-key")
        );
        let json: serde_json::Value = serde_json::from_str(body).unwrap();
        assert_eq!(json["id"], index + 1);
        assert_eq!(json["method"], method);
    }
}

#[tokio::test]
async fn enforces_response_limit() {
    let (url, server) = mock_server(vec![(200, "12345")]);
    let mut client = Client::new(&url, None).unwrap();
    client.set_max_response_bytes(4);
    assert!(matches!(
        client.execute(GetPowerStatus).await,
        Err(Error::ResponseTooLarge { limit: 4 })
    ));
    let requests = server.join().unwrap();
    assert!(!requests[0].to_ascii_lowercase().contains("x-auth-psk:"));
}

#[test]
fn rejects_invalid_configuration() {
    for url in [
        "https://example.com",
        "http://user@example.com",
        "http://example.com/path",
        "http://example.com/?q=1",
        "http://example.com/#fragment",
    ] {
        assert!(matches!(Client::new(url, None), Err(Error::InvalidBaseUrl)));
    }
    assert!(matches!(
        Client::new("http://127.0.0.1", Some("invalid\nkey")),
        Err(Error::InvalidPsk(_))
    ));
}
