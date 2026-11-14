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

/// Select the screen that supplies audio.
#[derive(Clone, Debug, Serialize)]
pub struct SetAudioSourceScreen {
    pub screen: String,
}

impl SonyBraviaTvCommand for SetAudioSourceScreen {
    const SERVICE: &str = "videoScreen";
    const METHOD: &str = "setAudioSourceScreen";
    const VERSION: &str = "1.0";

    type Params = Self;
    type Response = ();

    fn params(&self) -> Option<Self::Params> {
        Some(self.clone())
    }

    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        empty(result)
    }
}

/// Set a device-supported banner mode.
#[derive(Clone, Debug, Serialize)]
pub struct SetBannerMode {
    pub value: String,
}

impl SonyBraviaTvCommand for SetBannerMode {
    const SERVICE: &str = "videoScreen";
    const METHOD: &str = "setBannerMode";
    const VERSION: &str = "1.0";

    type Params = Self;
    type Response = ();

    fn params(&self) -> Option<Self::Params> {
        Some(self.clone())
    }

    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        empty(result)
    }
}

/// Change the multi-screen mode.
#[derive(Clone, Debug, Serialize)]
pub struct SetMultiScreenMode {
    pub mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option: Option<MultiScreenModeOption>,
}

impl SonyBraviaTvCommand for SetMultiScreenMode {
    const SERVICE: &str = "videoScreen";
    const METHOD: &str = "setMultiScreenMode";
    const VERSION: &str = "1.0";

    type Params = Self;
    type Response = ();

    fn params(&self) -> Option<Self::Params> {
        Some(self.clone())
    }

    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        empty(result)
    }
}

/// Change the picture-in-picture sub-screen position.
#[derive(Clone, Debug, Serialize)]
pub struct SetPipSubScreenPosition {
    pub position: String,
}

impl SonyBraviaTvCommand for SetPipSubScreenPosition {
    const SERVICE: &str = "videoScreen";
    const METHOD: &str = "setPipSubScreenPosition";
    const VERSION: &str = "1.0";

    type Params = Self;
    type Response = ();

    fn params(&self) -> Option<Self::Params> {
        Some(self.clone())
    }

    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        empty(result)
    }
}

/// Change the size of a picture-and-picture screen.
#[derive(Clone, Debug, Serialize)]
pub struct SetPapScreenSize {
    pub screen: String,
    pub size: String,
}

impl SonyBraviaTvCommand for SetPapScreenSize {
    const SERVICE: &str = "videoScreen";
    const METHOD: &str = "setPapScreenSize";
    const VERSION: &str = "1.0";

    type Params = Self;
    type Response = ();

    fn params(&self) -> Option<Self::Params> {
        Some(self.clone())
    }

    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        empty(result)
    }
}

/// Request notification of screen-state changes; this may be a long-poll request.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestToNotifyScreenState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_screen_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pip_sub_screen_position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_source_screen: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option: Option<MultiScreenModeOption>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ScreenState {
    pub multi_screen_mode: Option<String>,
    pub pip_sub_screen_position: Option<String>,
    pub audio_source_screen: Option<String>,
    pub option: Option<MultiScreenModeOption>,
}

impl SonyBraviaTvCommand for RequestToNotifyScreenState {
    const SERVICE: &str = "videoScreen";
    const METHOD: &str = "requestToNotifyScreenState";
    const VERSION: &str = "1.1";

    type Params = Self;
    type Response = ScreenState;

    fn params(&self) -> Option<Self::Params> {
        Some(self.clone())
    }

    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        single(result)
    }
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
