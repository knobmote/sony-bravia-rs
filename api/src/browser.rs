//! Browser service methods advertised by some BRAVIA TVs.

use super::*;

/// Read the browser's current page information.
#[derive(Clone, Copy, Debug, Default)]
pub struct GetTextUrl;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct BrowserPage {
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
}

impl SonyBraviaTvCommand for GetTextUrl {
    const SERVICE: &str = "browser";
    const METHOD: &str = "getTextUrl";
    const VERSION: &str = "1.0";

    type Params = ();
    type Response = BrowserPage;

    fn params(&self) -> Option<Self::Params> {
        None
    }

    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        single(result)
    }
}

/// Navigate the browser to a page. Available fields depend on the device.
#[derive(Clone, Debug, Serialize)]
pub struct SetTextUrl {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
}

impl SonyBraviaTvCommand for SetTextUrl {
    const SERVICE: &str = "browser";
    const METHOD: &str = "setTextUrl";
    const VERSION: &str = "1.0";

    type Params = Self;
    type Response = ();

    fn params(&self) -> Option<Self::Params> {
        Some(self.clone())
    }

    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        zero(result)
    }
}

/// Send a device-specific browser control value.
#[derive(Clone, Debug, Serialize)]
pub struct ActBrowserControl {
    pub control: String,
}

impl SonyBraviaTvCommand for ActBrowserControl {
    const SERVICE: &str = "browser";
    const METHOD: &str = "actBrowserControl";
    const VERSION: &str = "1.0";

    type Params = Self;
    type Response = ();

    fn params(&self) -> Option<Self::Params> {
        Some(self.clone())
    }

    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        zero(result)
    }
}
