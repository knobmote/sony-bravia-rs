#![no_std]

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use core::marker::PhantomData;
use core::num::NonZeroI32;

use serde::Deserialize;
use serde::Serialize;
use serde::de;
use serde_json as json;

use commands::empty;
use commands::single;
use commands::zero;

pub use error::Error;
pub use error::ErrorCode;
pub use system::GetPowerStatus;
pub use system::PowerStatus;
pub use system::SetPowerStatus;
pub use transport::Request;
pub use transport::Response;

pub mod app_control;
pub mod audio;
pub mod av_content;
pub mod browser;
pub mod cec;
pub mod discovery;
pub mod encryption;
pub mod guide;
pub mod system;
pub mod video;
pub mod video_screen;

mod commands;
mod error;
mod transport;

pub trait SonyBraviaTvCommand {
    /// Sony REST service used to route this command.
    const SERVICE: &str;
    const METHOD: &str;
    const VERSION: &str;

    type Params: Serialize;
    type Response: for<'de> Deserialize<'de>;

    fn params(&self) -> Option<Self::Params>;

    /// Decode the complete `result` array into the command's response.
    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response>;
}
