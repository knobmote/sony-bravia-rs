//! Input and content browsing commands.

use super::*;

/// List the TV's favorite content sources, which are not necessarily apps.
#[derive(Clone, Copy, Debug, Default)]
pub struct GetFavoriteList;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct FavoriteSource {
    pub source: String,
    pub label: String,
}

impl SonyBraviaTvCommand for GetFavoriteList {
    const SERVICE: &str = "avContent";
    const METHOD: &str = "getFavoriteList";
    const VERSION: &str = "1.0";
    type Params = ();
    type Response = Vec<FavoriteSource>;
    fn params(&self) -> Option<Self::Params> {
        None
    }
    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        single(result)
    }
}

/// Get the expanded external input status, including the device's status value.
#[derive(Clone, Copy, Debug, Default)]
pub struct GetCurrentExternalInputsStatusV1_1;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct ExternalInputV1_1 {
    pub uri: String,
    pub title: String,
    pub connection: bool,
    pub label: String,
    #[serde(default)]
    pub icon: String,
    pub status: Option<String>,
}

impl SonyBraviaTvCommand for GetCurrentExternalInputsStatusV1_1 {
    const SERVICE: &str = "avContent";
    const METHOD: &str = "getCurrentExternalInputsStatus";
    const VERSION: &str = "1.1";
    type Params = ();
    type Response = Vec<ExternalInputV1_1>;
    fn params(&self) -> Option<Self::Params> {
        None
    }
    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        single(result)
    }
}

/// Switch to a content or input URI obtained from the display.
#[derive(Clone, Debug, Serialize)]
pub struct SetPlayContent {
    pub uri: String,
}

impl SonyBraviaTvCommand for SetPlayContent {
    const SERVICE: &str = "avContent";
    const METHOD: &str = "setPlayContent";
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

/// Get the status of all external inputs.
#[derive(Clone, Copy, Debug, Default)]
pub struct GetCurrentExternalInputsStatus;

/// Information about an external input.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct ExternalInput {
    pub uri: String,
    pub title: String,
    pub connection: bool,
    pub label: String,
    #[serde(default)]
    pub icon: String,
}

impl SonyBraviaTvCommand for GetCurrentExternalInputsStatus {
    const SERVICE: &str = "avContent";
    const METHOD: &str = "getCurrentExternalInputsStatus";
    const VERSION: &str = "1.0";

    type Params = ();
    type Response = Vec<ExternalInput>;

    fn params(&self) -> Option<Self::Params> {
        None
    }

    fn decode_response(value: Vec<json::Value>) -> json::Result<Self::Response> {
        single(value)
    }
}

/// Get information about the currently playing content.
#[derive(Clone, Copy, Debug, Default)]
pub struct GetPlayingContentInfo;

/// The currently playing source and content.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct PlayingContent {
    pub uri: String,
    pub source: String,
    pub title: String,
}

impl SonyBraviaTvCommand for GetPlayingContentInfo {
    const SERVICE: &str = "avContent";
    const METHOD: &str = "getPlayingContentInfo";
    const VERSION: &str = "1.0";

    type Params = ();
    type Response = PlayingContent;

    fn params(&self) -> Option<Self::Params> {
        None
    }

    fn decode_response(value: Vec<json::Value>) -> json::Result<Self::Response> {
        single(value)
    }
}

/// List available URI schemes.
#[derive(Clone, Copy, Debug, Default)]
pub struct GetSchemeList;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Scheme {
    pub scheme: String,
}

impl SonyBraviaTvCommand for GetSchemeList {
    const SERVICE: &str = "avContent";
    const METHOD: &str = "getSchemeList";
    const VERSION: &str = "1.0";

    type Params = ();
    type Response = Vec<Scheme>;

    fn params(&self) -> Option<Self::Params> {
        None
    }

    fn decode_response(value: Vec<json::Value>) -> json::Result<Self::Response> {
        single(value)
    }
}

/// List sources in a URI scheme.
#[derive(Clone, Debug, Serialize)]
pub struct GetSourceList {
    pub scheme: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Source {
    pub source: String,
}

impl SonyBraviaTvCommand for GetSourceList {
    const SERVICE: &str = "avContent";
    const METHOD: &str = "getSourceList";
    const VERSION: &str = "1.0";

    type Params = Self;
    type Response = Vec<Source>;

    fn params(&self) -> Option<Self::Params> {
        Some(self.clone())
    }

    fn decode_response(value: Vec<json::Value>) -> json::Result<Self::Response> {
        single(value)
    }
}

/// Count content entries in a source.
#[derive(Clone, Debug, Serialize)]
pub struct GetContentCount {
    pub source: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct ContentCount {
    pub count: i32,
}

impl SonyBraviaTvCommand for GetContentCount {
    const SERVICE: &str = "avContent";
    const METHOD: &str = "getContentCount";
    const VERSION: &str = "1.0";

    type Params = Self;
    type Response = ContentCount;

    fn params(&self) -> Option<Self::Params> {
        Some(self.clone())
    }

    fn decode_response(value: Vec<json::Value>) -> json::Result<Self::Response> {
        single(value)
    }
}

/// Retrieve a page of content from a source; omitted options use device defaults.
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetContentList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "stIdx", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<u32>,
    #[serde(rename = "cnt", skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}

/// An item returned by content browsing.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Content {
    pub uri: String,
    pub title: Option<String>,
    #[serde(default)]
    pub index: i32,
}

impl SonyBraviaTvCommand for GetContentList {
    const SERVICE: &str = "avContent";
    const METHOD: &str = "getContentList";
    const VERSION: &str = "1.5";

    type Params = Self;
    type Response = Vec<Content>;

    fn params(&self) -> Option<Self::Params> {
        Some(self.clone())
    }

    fn decode_response(value: Vec<json::Value>) -> json::Result<Self::Response> {
        single(value)
    }
}
