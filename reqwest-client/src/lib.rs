//! Async HTTP transport for the Sony BRAVIA sans-I/O client.

use std::time::Duration;

use reqwest::Url;
use reqwest::header::HeaderValue;
use reqwest::header::InvalidHeaderValue;
use reqwest::redirect::Policy;
use sony_bravia_api::SonyBraviaTvCommand;

/// An HTTP client for one display. Requests are made sequentially per client.
#[derive(Debug)]
pub struct Client {
    base_url: Url,
    psk: Option<HeaderValue>,
    http: reqwest::Client,
    protocol: sony_bravia_client::Client,
    max_response_bytes: usize,
}

impl Client {
    /// Connect to an HTTP origin with an optional Sony pre-shared key.
    ///
    /// HTTP sends the key unencrypted. The client disables proxies and redirects,
    /// and applies a ten-second request timeout.
    ///
    /// # Errors
    /// Returns an error for an invalid origin or key, or if the HTTP client cannot be built.
    pub fn new(base_url: &str, psk: Option<&str>) -> Result<Self, Error> {
        let base_url = Url::parse(base_url).map_err(|_| Error::InvalidBaseUrl)?;
        if base_url.scheme() != "http"
            || base_url.host().is_none()
            || !base_url.username().is_empty()
            || base_url.password().is_some()
            || base_url.path() != "/"
            || base_url.query().is_some()
            || base_url.fragment().is_some()
        {
            return Err(Error::InvalidBaseUrl);
        }
        let psk = psk.map(HeaderValue::from_str).transpose()?;
        let psk = psk.map(|mut value| {
            value.set_sensitive(true);
            value
        });
        let http = reqwest::Client::builder()
            .redirect(Policy::none())
            .no_proxy()
            .timeout(Duration::from_secs(10))
            .build()?;
        Ok(Self {
            base_url,
            psk,
            http,
            protocol: sony_bravia_client::Client::new(),
            max_response_bytes: 1024 * 1024,
        })
    }

    /// Set the maximum accepted response body size, in bytes (default: 1 MiB).
    pub fn set_max_response_bytes(&mut self, limit: usize) {
        self.max_response_bytes = limit;
    }

    /// Send any supported typed command and decode its response.
    ///
    /// # Errors
    /// Returns an error if preparation, HTTP transport, or response decoding fails.
    pub async fn execute<C: SonyBraviaTvCommand>(
        &mut self,
        command: C,
    ) -> Result<C::Response, Error> {
        let pending = self.protocol.prepare(command)?;
        let mut url = self.base_url.clone();
        url.set_path(pending.path());
        let mut request = self
            .http
            .post(url)
            .header("content-type", "application/json");
        if let Some(psk) = &self.psk {
            request = request.header("X-Auth-PSK", psk);
        }
        let mut response = request.body(pending.body().to_owned()).send().await?;
        let status = response.status().as_u16();
        let mut body = Vec::new();
        while let Some(chunk) = response.chunk().await? {
            if chunk.len() > self.max_response_bytes.saturating_sub(body.len()) {
                return Err(Error::ResponseTooLarge {
                    limit: self.max_response_bytes,
                });
            }
            body.extend_from_slice(&chunk);
        }
        Ok(pending.complete(status, &body)?)
    }
}

/// Error from configuring the client, performing HTTP, or decoding a response.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The TV address must be a plain HTTP origin without credentials or a path.
    #[error("TV URL must be an HTTP origin, e.g. http://192.168.1.100")]
    InvalidBaseUrl,
    /// The pre-shared key is not a valid HTTP header value.
    #[error("invalid pre-shared key")]
    InvalidPsk(#[from] InvalidHeaderValue),
    /// An HTTP request or response body failed.
    #[error("HTTP transport failed: {0}")]
    Transport(#[from] reqwest::Error),
    /// The configured response body limit was exceeded.
    #[error("response exceeded the {limit}-byte limit")]
    ResponseTooLarge { limit: usize },
    /// The sans-I/O client could not prepare or interpret the request.
    #[error("BRAVIA request failed: {0}")]
    Protocol(#[from] sony_bravia_client::Error),
}
