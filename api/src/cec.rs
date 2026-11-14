//! HDMI-CEC and MHL configuration methods advertised by some BRAVIA TVs.

use super::*;

/// Enable or disable CEC control.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct SetCecControlMode {
    pub enabled: bool,
}

impl SonyBraviaTvCommand for SetCecControlMode {
    const SERVICE: &str = "cec";
    const METHOD: &str = "setCecControlMode";
    const VERSION: &str = "1.0";

    type Params = Self;
    type Response = ();

    fn params(&self) -> Option<Self::Params> {
        Some(*self)
    }

    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        empty(result)
    }
}

/// Enable or disable MHL power feed.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct SetMhlPowerFeedMode {
    pub enabled: bool,
}

impl SonyBraviaTvCommand for SetMhlPowerFeedMode {
    const SERVICE: &str = "cec";
    const METHOD: &str = "setMhlPowerFeedMode";
    const VERSION: &str = "1.0";

    type Params = Self;
    type Response = ();

    fn params(&self) -> Option<Self::Params> {
        Some(*self)
    }

    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        empty(result)
    }
}

/// Configure CEC power synchronization.
#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPowerSyncMode {
    pub sink_power_off_sync: bool,
    pub source_power_on_sync: bool,
}

impl SonyBraviaTvCommand for SetPowerSyncMode {
    const SERVICE: &str = "cec";
    const METHOD: &str = "setPowerSyncMode";
    const VERSION: &str = "1.0";

    type Params = Self;
    type Response = ();

    fn params(&self) -> Option<Self::Params> {
        Some(*self)
    }

    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        empty(result)
    }
}

/// Enable or disable automatic input switching for MHL.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct SetMhlAutoInputChangeMode {
    pub enabled: bool,
}

impl SonyBraviaTvCommand for SetMhlAutoInputChangeMode {
    const SERVICE: &str = "cec";
    const METHOD: &str = "setMhlAutoInputChangeMode";
    const VERSION: &str = "1.0";

    type Params = Self;
    type Response = ();

    fn params(&self) -> Option<Self::Params> {
        Some(*self)
    }

    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        empty(result)
    }
}
