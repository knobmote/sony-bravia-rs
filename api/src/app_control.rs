//! Application discovery and control commands.

use super::*;

/// Read the text in the active software keyboard using the v1.1 API.
///
/// An empty key requests unencrypted text. Sony's reference contains conflicting
/// optionality guidance, so the key field is always sent.
#[derive(Clone, Debug, Serialize)]
pub struct GetTextForm {
    #[serde(rename = "encKey")]
    pub encryption_key: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct TextForm {
    pub text: String,
}

impl SonyBraviaTvCommand for GetTextForm {
    const SERVICE: &str = "appControl";
    const METHOD: &str = "getTextForm";
    const VERSION: &str = "1.1";
    type Params = Self;
    type Response = TextForm;
    fn params(&self) -> Option<Self::Params> {
        Some(self.clone())
    }
    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        single(result)
    }
}

/// Set software keyboard text using the v1.1 API.
///
/// When a key is supplied, `text` must already be encrypted as the device expects.
#[derive(Clone, Debug, Serialize)]
pub struct SetTextFormV1_1 {
    #[serde(rename = "encKey")]
    pub encryption_key: String,
    pub text: String,
}

impl SonyBraviaTvCommand for SetTextFormV1_1 {
    const SERVICE: &str = "appControl";
    const METHOD: &str = "setTextForm";
    const VERSION: &str = "1.1";
    type Params = Self;
    type Response = ();
    fn params(&self) -> Option<Self::Params> {
        Some(self.clone())
    }
    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        empty(result)
    }
}

/// List launchable applications.
#[derive(Clone, Copy, Debug, Default)]
pub struct GetApplicationList;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Application {
    pub title: String,
    pub uri: String,
    #[serde(default)]
    pub icon: String,
}

impl SonyBraviaTvCommand for GetApplicationList {
    const SERVICE: &str = "appControl";
    const METHOD: &str = "getApplicationList";
    const VERSION: &str = "1.0";

    type Params = ();
    type Response = Vec<Application>;

    fn params(&self) -> Option<Self::Params> {
        None
    }

    fn decode_response(value: Vec<json::Value>) -> json::Result<Self::Response> {
        single(value)
    }
}

/// List application-related status values.
#[derive(Clone, Copy, Debug, Default)]
pub struct GetApplicationStatusList;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct ApplicationStatus {
    pub name: String,
    pub status: String,
}

impl SonyBraviaTvCommand for GetApplicationStatusList {
    const SERVICE: &str = "appControl";
    const METHOD: &str = "getApplicationStatusList";
    const VERSION: &str = "1.0";

    type Params = ();
    type Response = Vec<ApplicationStatus>;

    fn params(&self) -> Option<Self::Params> {
        None
    }

    fn decode_response(value: Vec<json::Value>) -> json::Result<Self::Response> {
        single(value)
    }
}

/// Launch an application using its URI.
#[derive(Clone, Debug, Serialize)]
pub struct SetActiveApp {
    pub uri: String,
}

impl SonyBraviaTvCommand for SetActiveApp {
    const SERVICE: &str = "appControl";
    const METHOD: &str = "setActiveApp";
    const VERSION: &str = "1.0";

    type Params = Self;
    type Response = ();

    fn params(&self) -> Option<Self::Params> {
        Some(self.clone())
    }

    fn decode_response(value: Vec<json::Value>) -> json::Result<Self::Response> {
        empty(value)
    }
}

/// Terminate applications on the display.
#[derive(Clone, Copy, Debug, Default)]
pub struct TerminateApps;

impl SonyBraviaTvCommand for TerminateApps {
    const SERVICE: &str = "appControl";
    const METHOD: &str = "terminateApps";
    const VERSION: &str = "1.0";

    type Params = ();
    type Response = ();

    fn params(&self) -> Option<Self::Params> {
        None
    }

    fn decode_response(value: Vec<json::Value>) -> json::Result<Self::Response> {
        empty(value)
    }
}

/// Read the status of the web application runtime.
#[derive(Clone, Copy, Debug, Default)]
pub struct GetWebAppStatus;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct WebAppStatus {
    pub active: bool,
    #[serde(default)]
    pub url: String,
}

impl SonyBraviaTvCommand for GetWebAppStatus {
    const SERVICE: &str = "appControl";
    const METHOD: &str = "getWebAppStatus";
    const VERSION: &str = "1.0";

    type Params = ();
    type Response = WebAppStatus;

    fn params(&self) -> Option<Self::Params> {
        None
    }

    fn decode_response(value: Vec<json::Value>) -> json::Result<Self::Response> {
        single(value)
    }
}

/// Enter unencrypted text into the active on-screen keyboard (API v1.0).
#[derive(Clone, Debug)]
pub struct SetTextForm {
    pub text: String,
}

impl SonyBraviaTvCommand for SetTextForm {
    const SERVICE: &str = "appControl";
    const METHOD: &str = "setTextForm";
    const VERSION: &str = "1.0";

    type Params = String;
    type Response = ();

    fn params(&self) -> Option<Self::Params> {
        Some(self.text.clone())
    }

    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        zero(result)
    }
}
