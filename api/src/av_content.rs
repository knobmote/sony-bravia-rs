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

/// Replace the contents of a favorite source.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetFavoriteContentList {
    pub fav_source: String,
    pub contents: Vec<String>,
}

impl SonyBraviaTvCommand for SetFavoriteContentList {
    const SERVICE: &str = "avContent";
    const METHOD: &str = "setFavoriteContentList";
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

/// Change delete protection on a content URI.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDeleteProtection {
    pub uri: String,
    pub is_protected: bool,
}

impl SonyBraviaTvCommand for SetDeleteProtection {
    const SERVICE: &str = "avContent";
    const METHOD: &str = "setDeleteProtection";
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

/// Delete a content item. This operation changes device state.
#[derive(Clone, Debug, Serialize)]
pub struct DeleteContent {
    pub uri: String,
}

impl SonyBraviaTvCommand for DeleteContent {
    const SERVICE: &str = "avContent";
    const METHOD: &str = "deleteContent";
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

/// Select a TV channel using the v1.0 channel identifier.
#[derive(Clone, Debug, Serialize)]
pub struct SetPlayTvContent {
    pub channel: String,
}

/// Select TV content using the v1.1 device-specific channel object.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPlayTvContentV1_1 {
    /// The TV advertises a `PlayTvChannel` object without its field definitions.
    pub channel: json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broadcast_freq_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_visibility_settings: Option<bool>,
}

impl SonyBraviaTvCommand for SetPlayTvContentV1_1 {
    const SERVICE: &str = "avContent";
    const METHOD: &str = "setPlayTvContent";
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

impl SonyBraviaTvCommand for SetPlayTvContent {
    const SERVICE: &str = "avContent";
    const METHOD: &str = "setPlayTvContent";
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

/// A device-specific content visibility update.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TvContentVisibility {
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epg_visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_surfing_visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

/// Change visibility for one or more TV content items.
#[derive(Clone, Debug)]
pub struct SetTvContentVisibility {
    pub items: Vec<TvContentVisibility>,
}

impl SonyBraviaTvCommand for SetTvContentVisibility {
    const SERVICE: &str = "avContent";
    const METHOD: &str = "setTvContentVisibility";
    const VERSION: &str = "1.0";

    type Params = Vec<TvContentVisibility>;
    type Response = ();

    fn params(&self) -> Option<Self::Params> {
        Some(self.items.clone())
    }

    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        empty(result)
    }
}

/// Read device-supported parental-rating settings.
#[derive(Clone, Copy, Debug, Default)]
pub struct GetParentalRatingSettings;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ParentalRatingSettings {
    pub rating_type_age: Option<i32>,
    pub rating_type_sony: Option<String>,
    pub rating_country: Option<String>,
    #[serde(rename = "ratingCustomTypeTV")]
    pub rating_custom_type_tv: Option<Vec<String>>,
    #[serde(rename = "ratingCustomTypeMpaa")]
    pub rating_custom_type_mpaa: Option<String>,
    #[serde(rename = "ratingCustomTypeCaEnglish")]
    pub rating_custom_type_ca_english: Option<String>,
    #[serde(rename = "ratingCustomTypeCaFrench")]
    pub rating_custom_type_ca_french: Option<String>,
    pub unrated_lock: Option<bool>,
}

impl SonyBraviaTvCommand for GetParentalRatingSettings {
    const SERVICE: &str = "avContent";
    const METHOD: &str = "getParentalRatingSettings";
    const VERSION: &str = "1.0";

    type Params = ();
    type Response = ParentalRatingSettings;

    fn params(&self) -> Option<Self::Params> {
        None
    }

    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        single(result)
    }
}

/// Count entries in a source with optional type and target filters (v1.1).
#[derive(Clone, Debug, Serialize)]
pub struct GetContentCountV1_1 {
    pub source: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

impl SonyBraviaTvCommand for GetContentCountV1_1 {
    const SERVICE: &str = "avContent";
    const METHOD: &str = "getContentCount";
    const VERSION: &str = "1.1";

    type Params = Self;
    type Response = ContentCount;

    fn params(&self) -> Option<Self::Params> {
        Some(self.clone())
    }

    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        single(result)
    }
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

/// Retrieve content from a source using the v1.0 API.
#[derive(Clone, Debug, Serialize)]
pub struct GetContentListV1_0 {
    pub source: String,
    #[serde(rename = "stIdx", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<u32>,
    #[serde(rename = "cnt", skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

impl SonyBraviaTvCommand for GetContentListV1_0 {
    const SERVICE: &str = "avContent";
    const METHOD: &str = "getContentList";
    const VERSION: &str = "1.0";

    type Params = Self;
    type Response = Vec<Content>;

    fn params(&self) -> Option<Self::Params> {
        Some(self.clone())
    }

    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        single(result)
    }
}

/// Retrieve content from a source using the v1.2 API.
#[derive(Clone, Debug, Serialize)]
pub struct GetContentListV1_2 {
    pub source: String,
    #[serde(rename = "stIdx", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<u32>,
    #[serde(rename = "cnt", skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

impl SonyBraviaTvCommand for GetContentListV1_2 {
    const SERVICE: &str = "avContent";
    const METHOD: &str = "getContentList";
    const VERSION: &str = "1.2";

    type Params = Self;
    type Response = Vec<Content>;

    fn params(&self) -> Option<Self::Params> {
        Some(self.clone())
    }

    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        single(result)
    }
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
