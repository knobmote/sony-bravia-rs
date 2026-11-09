use super::*;

#[derive(Clone, Debug, Deserialize)]
pub struct Error {
    pub code: ErrorCode,
    pub message: String,
}

impl Error {
    pub fn from_json(text: &str) -> json::Result<Self> {
        json::from_str(text)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(f)
    }
}

// Keep the wire values and conversions together so the mappings cannot drift.
macro_rules! error_codes {
    ($( $(#[$meta:meta])* $name:ident = $value:literal, )*) => {
        /// A REST API error code. Unknown values are retained for newer devices.
        ///
        /// See Sony's [error code reference](https://pro-bravia.sony.net/remote-display-control/rest-api/reference/).
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(from = "u16", into = "u16")]
        pub enum ErrorCode {
            $( $(#[$meta])* $name, )*
            /// A code not listed in the reference.
            Unknown(u16),
        }

        impl From<u16> for ErrorCode {
            fn from(value: u16) -> Self {
                match value {
                    $( $value => Self::$name, )*
                    value => Self::Unknown(value),
                }
            }
        }

        impl From<ErrorCode> for u16 {
            fn from(code: ErrorCode) -> Self {
                match code {
                    $( ErrorCode::$name => $value, )*
                    ErrorCode::Unknown(value) => value,
                }
            }
        }

        impl ErrorCode {
            pub fn from_json(text: &str) -> json::Result<Self> {
                json::from_str(text)
            }

            pub fn to_json(&self) -> json::Result<String> {
                json::to_string(self)
            }
        }
    };
}

error_codes! {
    /// Generic error.
    Any = 1,
    /// The server could not reply in time.
    Timeout = 2,
    /// Request parameters do not follow the API specification.
    IllegalArgument = 3,
    /// The request body is malformed or missing required fields.
    IllegalRequest = 5,
    /// The server cannot handle the request in its current state.
    IllegalState = 7,
    /// The requested API does not exist.
    NoSuchMethod = 12,
    /// The requested API version is unsupported.
    UnsupportedVersion = 14,
    /// The requested operation is unsupported for the given parameters.
    UnsupportedOperation = 15,
    /// Authentication is required.
    Unauthorized = 401,
    /// Access is forbidden.
    Forbidden = 403,
    /// No supported API version matched the request.
    NotFound = 404,
    /// The request body is too large.
    RequestEntityTooLarge = 413,
    /// The request URI is too long.
    RequestUriTooLong = 414,
    /// The request method is not implemented.
    NotImplemented = 501,
    /// The server is temporarily unavailable.
    ServiceUnavailable = 503,
    /// A long-polling request timed out.
    RequestRetry = 40000,
    /// The maximum number of long-polling clients was exceeded.
    ClientOverMaximum = 40001,
    /// Encryption or decryption failed.
    EncryptionFailed = 40002,
    /// A previous request is still awaiting a response.
    RequestDuplicated = 40003,
    /// One or more settings failed; read the settings to identify them.
    MultipleSettingsFailed = 40004,
    /// The display is turned off.
    DisplayTurnedOff = 40005,
    /// A general service error; the message contains diagnostic details.
    GeneralServiceError = 40006,
    /// The password has expired.
    PasswordExpired = 40200,
    /// The method requires AC power.
    AcPowerRequired = 40201,
    /// Another screen change request is in progress.
    ScreenChangeInProgress = 40600,
    /// The audio target is unsupported or cannot be controlled.
    TargetNotSupported = 40800,
    /// The requested volume is out of range.
    VolumeOutOfRange = 40801,
    /// The content is protected.
    ContentProtected = 41000,
    /// The content does not exist.
    ContentDoesNotExist = 41001,
    /// The storage has no content.
    StorageHasNoContent = 41002,
    /// Some content could not be deleted.
    ContentDeletionFailed = 41003,
    /// USB recording fixes the current channel.
    ChannelFixedByUsbRecording = 41011,
    /// SCART recording fixes the current channel.
    ChannelFixedByScartRecording = 41012,
    /// The chapter does not exist.
    ChapterDoesNotExist = 41013,
    /// The channel cannot be uniquely determined.
    ChannelNotUnique = 41014,
    /// The channel list is empty.
    EmptyChannelList = 41015,
    /// The storage does not exist.
    StorageDoesNotExist = 41020,
    /// The storage is full.
    StorageFull = 41021,
    /// Setting a content attribute failed.
    ContentAttributeSettingFailed = 41022,
    /// The group ID is unknown.
    UnknownGroupId = 41023,
    /// The content is unsupported.
    ContentNotSupported = 41024,
    /// Another application request is in progress.
    AnotherRequestInProgress = 41400,
    /// Launching the application failed.
    AppLaunchFailed = 41401,
    /// The application launch was accepted but its completion is unknown.
    AppLaunchInProgress = 41402,
    /// Some applications could not be terminated.
    AppTerminationFailed = 41403,
    /// The encryption key does not exist yet.
    KeyDoesNotExist = 42400,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn documented_codes_round_trip() {
        let cases = [
            (1, ErrorCode::Any),
            (2, ErrorCode::Timeout),
            (3, ErrorCode::IllegalArgument),
            (5, ErrorCode::IllegalRequest),
            (7, ErrorCode::IllegalState),
            (12, ErrorCode::NoSuchMethod),
            (14, ErrorCode::UnsupportedVersion),
            (15, ErrorCode::UnsupportedOperation),
            (401, ErrorCode::Unauthorized),
            (403, ErrorCode::Forbidden),
            (404, ErrorCode::NotFound),
            (413, ErrorCode::RequestEntityTooLarge),
            (414, ErrorCode::RequestUriTooLong),
            (501, ErrorCode::NotImplemented),
            (503, ErrorCode::ServiceUnavailable),
            (40000, ErrorCode::RequestRetry),
            (40001, ErrorCode::ClientOverMaximum),
            (40002, ErrorCode::EncryptionFailed),
            (40003, ErrorCode::RequestDuplicated),
            (40004, ErrorCode::MultipleSettingsFailed),
            (40005, ErrorCode::DisplayTurnedOff),
            (40006, ErrorCode::GeneralServiceError),
            (40200, ErrorCode::PasswordExpired),
            (40201, ErrorCode::AcPowerRequired),
            (40600, ErrorCode::ScreenChangeInProgress),
            (40800, ErrorCode::TargetNotSupported),
            (40801, ErrorCode::VolumeOutOfRange),
            (41000, ErrorCode::ContentProtected),
            (41001, ErrorCode::ContentDoesNotExist),
            (41002, ErrorCode::StorageHasNoContent),
            (41003, ErrorCode::ContentDeletionFailed),
            (41011, ErrorCode::ChannelFixedByUsbRecording),
            (41012, ErrorCode::ChannelFixedByScartRecording),
            (41013, ErrorCode::ChapterDoesNotExist),
            (41014, ErrorCode::ChannelNotUnique),
            (41015, ErrorCode::EmptyChannelList),
            (41020, ErrorCode::StorageDoesNotExist),
            (41021, ErrorCode::StorageFull),
            (41022, ErrorCode::ContentAttributeSettingFailed),
            (41023, ErrorCode::UnknownGroupId),
            (41024, ErrorCode::ContentNotSupported),
            (41400, ErrorCode::AnotherRequestInProgress),
            (41401, ErrorCode::AppLaunchFailed),
            (41402, ErrorCode::AppLaunchInProgress),
            (41403, ErrorCode::AppTerminationFailed),
            (42400, ErrorCode::KeyDoesNotExist),
        ];

        let mut buf = std::fmt::NumBuffer::new();

        for (number, code) in cases {
            assert_eq!(ErrorCode::from(number), code);
            assert_eq!(u16::from(code), number);
            assert_eq!(code.to_json().unwrap(), number.to_string());
            let text = number.format_into(&mut buf);
            assert_eq!(ErrorCode::from_json(text).unwrap(), code);
        }
    }

    #[test]
    fn error_shape_and_unknown_codes() {
        let error = Error::from_json(r#"[40006, "E1234"]"#).unwrap();
        assert_eq!(error.code, ErrorCode::GeneralServiceError);
        assert_eq!(error.message, "E1234");

        for number in [0, 418, 65535] {
            let json = format!(r#"[{number}, "device message"]"#);
            let error = Error::from_json(&json).unwrap();
            assert_eq!(error.code, ErrorCode::Unknown(number));
            assert_eq!(error.message, "device message");
            assert_eq!(error.code.to_json().unwrap(), number.to_string());
        }

        for json in [r#"[-1, "bad"]"#, r#"[65536, "bad"]"#, r#"["1", "bad"]"#] {
            assert!(Error::from_json(json).is_err());
        }
    }
}
