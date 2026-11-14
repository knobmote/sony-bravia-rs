//! Scene controls provided by the videoScreen service.

use super::*;

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

    fn decode_response(value: Value) -> json::Result<Self::Response> {
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

    fn decode_response(value: Value) -> json::Result<Self::Response> {
        empty(value)
    }
}
