//! Scene controls provided by the videoScreen service.

use super::*;

/// Read which screen supplies audio.
#[derive(Clone, Copy, Debug, Default)]
pub struct GetAudioSourceScreen;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct AudioSourceScreen {
    pub screen: String,
}

impl SonyBraviaTvCommand for GetAudioSourceScreen {
    const SERVICE: &str = "videoScreen";
    const METHOD: &str = "getAudioSourceScreen";
    const VERSION: &str = "1.0";
    type Params = ();
    type Response = AudioSourceScreen;
    fn params(&self) -> Option<Self::Params> {
        None
    }
    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        single(result)
    }
}

/// Read the current banner mode.
#[derive(Clone, Copy, Debug, Default)]
pub struct GetBannerMode;

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BannerMode {
    pub current_value: String,
    /// Candidate details are device-specific.
    #[serde(default)]
    pub candidate: Vec<json::Value>,
}

impl SonyBraviaTvCommand for GetBannerMode {
    const SERVICE: &str = "videoScreen";
    const METHOD: &str = "getBannerMode";
    const VERSION: &str = "1.0";
    type Params = ();
    type Response = BannerMode;
    fn params(&self) -> Option<Self::Params> {
        None
    }
    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        single(result)
    }
}

/// Read the picture-in-picture sub-screen position.
#[derive(Clone, Copy, Debug, Default)]
pub struct GetPipSubScreenPosition;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct PipSubScreenPosition {
    pub position: String,
}

impl SonyBraviaTvCommand for GetPipSubScreenPosition {
    const SERVICE: &str = "videoScreen";
    const METHOD: &str = "getPipSubScreenPosition";
    const VERSION: &str = "1.0";
    type Params = ();
    type Response = PipSubScreenPosition;
    fn params(&self) -> Option<Self::Params> {
        None
    }
    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        single(result)
    }
}

/// Read the current multi-screen mode.
#[derive(Clone, Copy, Debug, Default)]
pub struct GetMultiScreenMode;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct MultiScreenMode {
    pub mode: String,
    pub option: Option<MultiScreenModeOption>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MultiScreenModeOption {
    #[serde(rename = "internetTVMode", skip_serializing_if = "Option::is_none")]
    pub internet_tv_mode: Option<bool>,
}

impl SonyBraviaTvCommand for GetMultiScreenMode {
    const SERVICE: &str = "videoScreen";
    const METHOD: &str = "getMultiScreenMode";
    const VERSION: &str = "1.1";
    type Params = ();
    type Response = MultiScreenMode;
    fn params(&self) -> Option<Self::Params> {
        None
    }
    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        single(result)
    }
}

/// Retrieve the current scene and available candidates.
#[derive(Clone, Copy, Debug, Default)]
pub struct GetSceneSetting;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SceneSetting {
    pub current_value: String,
    pub candidate: Option<Vec<SceneCandidate>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct SceneCandidate {
    #[serde(default)]
    pub value: String,
}

impl SonyBraviaTvCommand for GetSceneSetting {
    const SERVICE: &str = "videoScreen";
    const METHOD: &str = "getSceneSetting";
    const VERSION: &str = "1.0";

    type Params = ();
    type Response = SceneSetting;

    fn params(&self) -> Option<Self::Params> {
        None
    }

    fn decode_response(value: Vec<json::Value>) -> json::Result<Self::Response> {
        single(value)
    }
}

/// Select a scene reported by the display.
#[derive(Clone, Debug, Serialize)]
pub struct SetSceneSetting {
    pub value: String,
}

impl SonyBraviaTvCommand for SetSceneSetting {
    const SERVICE: &str = "videoScreen";
    const METHOD: &str = "setSceneSetting";
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
