//! Picture and display rotation commands.

use super::*;

/// Get screen rotation. Sony restricts this API to localhost on certain models.
#[derive(Clone, Copy, Debug, Default)]
pub struct GetScreenRotation;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct ScreenRotation {
    pub rotation: u16,
}

impl SonyBraviaTvCommand for GetScreenRotation {
    const SERVICE: &str = "video";
    const METHOD: &str = "getScreenRotation";
    const VERSION: &str = "1.0";

    type Params = ();
    type Response = ScreenRotation;

    fn params(&self) -> Option<Self::Params> {
        None
    }

    fn decode_response(value: Vec<json::Value>) -> json::Result<Self::Response> {
        single(value)
    }
}

/// Set screen rotation in degrees. Sony restricts this API to localhost on certain models.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct SetScreenRotation {
    pub rotation: u16,
}

impl SonyBraviaTvCommand for SetScreenRotation {
    const SERVICE: &str = "video";
    const METHOD: &str = "setScreenRotation";
    const VERSION: &str = "1.0";

    type Params = Self;
    type Response = ();

    fn params(&self) -> Option<Self::Params> {
        Some(*self)
    }

    fn decode_response(value: Vec<json::Value>) -> json::Result<Self::Response> {
        empty(value)
    }
}

/// Get picture settings, optionally filtering by target.
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetPictureQualitySettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PictureQualitySetting {
    pub target: String,
    pub current_value: String,
    #[serde(default = "default_available")]
    pub is_available: bool,
    pub candidate: Option<Vec<PictureCandidate>>,
}

fn default_available() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct PictureCandidate {
    #[serde(default)]
    pub value: String,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub step: Option<f64>,
}

impl SonyBraviaTvCommand for GetPictureQualitySettings {
    const SERVICE: &str = "video";
    const METHOD: &str = "getPictureQualitySettings";
    const VERSION: &str = "1.0";

    type Params = Self;
    type Response = Vec<PictureQualitySetting>;

    fn params(&self) -> Option<Self::Params> {
        Some(self.clone())
    }

    fn decode_response(value: Vec<json::Value>) -> json::Result<Self::Response> {
        single(value)
    }
}

/// A picture-setting target and value to apply.
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct PictureSetting {
    pub target: String,
    pub value: String,
}

/// Change picture settings.
#[derive(Clone, Debug, Serialize)]
pub struct SetPictureQualitySettings {
    pub settings: Vec<PictureSetting>,
}

impl SonyBraviaTvCommand for SetPictureQualitySettings {
    const SERVICE: &str = "video";
    const METHOD: &str = "setPictureQualitySettings";
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
