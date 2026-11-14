//! Sans-I/O request preparation and response handling for Sony BRAVIA.
//!
//! The caller supplies the HTTP transport, destination, authentication, and
//! timeouts. A [`Pending`] request keeps the command type and request ID together
//! so that its response can be checked and decoded without exposing Sony's
//! response envelope to the caller.

#![no_std]

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::error;
use core::fmt;
use core::marker::PhantomData;
use core::num::NonZeroI32;

use sony_bravia_api::Request;
use sony_bravia_api::Response;
use sony_bravia_api::SonyBraviaTvCommand;

/// Prepares requests with unique positive IDs for one client instance.
///
/// IDs are not reused: create a new client when the ID space is exhausted, and
/// keep each client's pending requests associated with its own connection.
#[derive(Debug)]
pub struct Client {
    next_id: Option<NonZeroI32>,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            next_id: NonZeroI32::new(1),
        }
    }
}

impl Client {
    /// Create a client starting with request ID 1.
    pub fn new() -> Self {
        Self::default()
    }

    /// Prepare a command without performing I/O.
    ///
    /// # Errors
    /// Returns an error if the ID space is exhausted or parameters cannot be serialized.
    pub fn prepare<C: SonyBraviaTvCommand>(&mut self, command: C) -> Result<Pending<C>, Error> {
        let id = self.next_id.ok_or(Error::IdsExhausted)?;
        let body = Request::from_command(command, id)
            .and_then(|request| request.to_json())
            .map_err(Error::Encode)?;
        self.next_id = id.get().checked_add(1).and_then(NonZeroI32::new);
        Ok(Pending {
            id,
            path: format!("/sony/{}", C::SERVICE),
            body,
            command: PhantomData,
        })
    }
}

/// A prepared request and the information needed to decode its response.
#[derive(Debug)]
pub struct Pending<C> {
    id: NonZeroI32,
    path: String,
    body: String,
    command: PhantomData<fn() -> C>,
}

impl<C: SonyBraviaTvCommand> Pending<C> {
    /// The request ID, for diagnostics or transport correlation.
    pub fn id(&self) -> NonZeroI32 {
        self.id
    }

    /// The URL path to append to the display's base URL.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// The JSON body to send in an HTTP POST with `Content-Type: application/json`.
    pub fn body(&self) -> &str {
        &self.body
    }

    /// Interpret the HTTP status and complete response body for this request.
    ///
    /// HTTP failures retain the raw body and any Sony error it contains. A JSON
    /// response with a different ID is rejected even when the HTTP status is an error.
    ///
    /// # Errors
    /// Returns an HTTP, protocol, or device error if the request did not succeed.
    pub fn complete(&self, status: u16, body: &[u8]) -> Result<C::Response, Error> {
        if status != 200 {
            let device_error = match serde_json::from_slice::<Response>(body) {
                Ok(response) => {
                    self.check_id(response.id())?;
                    match response {
                        Response::Failure { error, .. } => Some(error),
                        Response::Success { .. } => None,
                    }
                }
                Err(_) => None,
            };
            return Err(Error::Http {
                status,
                body: body.to_vec(),
                device_error,
            });
        }

        let response = serde_json::from_slice::<Response>(body).map_err(Error::Decode)?;
        self.check_id(response.id())?;
        response
            .response::<C>()
            .map_err(Error::Decode)?
            .map_err(Error::Device)
    }

    fn check_id(&self, actual: NonZeroI32) -> Result<(), Error> {
        if actual == self.id {
            Ok(())
        } else {
            Err(Error::MismatchedId {
                expected: self.id,
                actual,
            })
        }
    }
}

/// Failure while preparing or interpreting a request.
#[derive(Debug)]
pub enum Error {
    /// This client has used every positive request ID.
    IdsExhausted,
    /// Command parameters could not be serialized.
    Encode(serde_json::Error),
    /// The response body or command-specific result was invalid.
    Decode(serde_json::Error),
    /// The response belongs to a different request.
    MismatchedId {
        expected: NonZeroI32,
        actual: NonZeroI32,
    },
    /// The display returned a REST API error with HTTP status 200.
    Device(sony_bravia_api::Error),
    /// The server returned an HTTP status other than 200.
    Http {
        status: u16,
        body: Vec<u8>,
        device_error: Option<sony_bravia_api::Error>,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IdsExhausted => write!(f, "request ID space exhausted"),
            Self::Encode(error) => write!(f, "failed to encode request: {error}"),
            Self::Decode(error) => write!(f, "failed to decode response: {error}"),
            Self::MismatchedId { expected, actual } => write!(
                f,
                "response ID {actual} does not match request ID {expected}"
            ),
            Self::Device(error) => write!(f, "display error: {error}"),
            Self::Http {
                status,
                device_error: Some(error),
                ..
            } => write!(f, "HTTP {status}: {error}"),
            Self::Http { status, .. } => write!(f, "HTTP {status}"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Encode(error) | Self::Decode(error) => Some(error),
            Self::Device(error)
            | Self::Http {
                device_error: Some(error),
                ..
            } => Some(error),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sony_bravia_api::ErrorCode;
    use sony_bravia_api::audio::SetAudioMute;
    use sony_bravia_api::system::GetPowerStatus;
    use sony_bravia_api::system::PowerStatus;

    #[test]
    fn prepares_and_matches_out_of_order_responses() {
        let mut client = Client::new();
        let power = client.prepare(GetPowerStatus).unwrap();
        let mute = client.prepare(SetAudioMute { status: true }).unwrap();
        assert_eq!(power.id().get(), 1);
        assert_eq!(power.path(), "/sony/system");
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(power.body()).unwrap(),
            serde_json::json!({"id":1,"method":"getPowerStatus","params":[],"version":"1.0"})
        );
        assert_eq!(mute.id().get(), 2);
        assert_eq!(mute.path(), "/sony/audio");
        assert_eq!(mute.complete(200, br#"{"id":2,"result":[0]}"#).unwrap(), ());
        assert_eq!(
            power
                .complete(200, br#"{"id":1,"result":[{"status":"active"}]}"#)
                .unwrap(),
            PowerStatus::Active
        );
        assert!(matches!(
            power.complete(200, br#"{"id":2,"result":[{"status":"active"}]}"#),
            Err(Error::MismatchedId { .. })
        ));
    }

    #[test]
    fn reports_device_protocol_and_http_errors() {
        let pending = Client::new().prepare(GetPowerStatus).unwrap();
        let error_body = br#"{"id":1,"error":[401,"Unauthorized"]}"#;
        assert!(
            matches!(pending.complete(200, error_body), Err(Error::Device(error)) if error.code == ErrorCode::Unauthorized)
        );
        assert!(
            matches!(pending.complete(401, error_body), Err(Error::Http { status: 401, device_error: Some(error), .. }) if error.code == ErrorCode::Unauthorized)
        );
        assert!(
            matches!(pending.complete(503, b"unavailable"), Err(Error::Http { status: 503, body, device_error: None }) if body == b"unavailable")
        );
        assert!(matches!(
            pending.complete(401, br#"{"id":9,"error":[401,"Unauthorized"]}"#),
            Err(Error::MismatchedId { .. })
        ));
        assert!(matches!(
            pending.complete(200, b"invalid"),
            Err(Error::Decode(_))
        ));
        assert!(matches!(
            pending.complete(200, br#"{"id":1,"result":[]}"#),
            Err(Error::Decode(_))
        ));
    }

    #[test]
    fn does_not_wrap_request_ids() {
        let mut client = Client {
            next_id: NonZeroI32::new(i32::MAX),
        };
        let pending = client.prepare(GetPowerStatus).unwrap();
        assert_eq!(pending.id().get(), i32::MAX);
        assert!(matches!(
            client.prepare(GetPowerStatus),
            Err(Error::IdsExhausted)
        ));
    }
}
