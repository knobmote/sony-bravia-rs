use std::fmt;
use std::num::NonZeroI32;

use serde::Deserialize;
use serde::Serialize;
use serde_json as json;

pub use error::Error;
pub use error::ErrorCode;
pub use transport::Request;
pub use transport::Response;

mod error;
mod transport;

pub trait SonyBraviaTvCommand {
    const METHOD: &str;
    const VERSION: &str;

    type Params: Serialize;
    type Response: for<'de> Deserialize<'de>;

    fn params(&self) -> Option<Self::Params>;
}
