//! Discovery of the services and API versions supported by a display.

use super::*;

/// Discover APIs, optionally restricting discovery to selected services.
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetSupportedApiInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct ServiceInfo {
    pub service: String,
    pub protocols: Vec<String>,
    pub apis: Vec<ApiInfo>,
    pub notifications: Option<Vec<ApiInfo>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct ApiInfo {
    pub name: String,
    pub versions: Vec<ApiVersion>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ApiVersion {
    pub version: String,
    pub protocols: Option<Vec<String>>,
    pub auth_level: Option<String>,
}

impl SonyBraviaTvCommand for GetSupportedApiInfo {
    const SERVICE: &str = "guide";
    const METHOD: &str = "getSupportedApiInfo";
    const VERSION: &str = "1.0";

    type Params = Self;
    type Response = Vec<ServiceInfo>;

    fn params(&self) -> Option<Self::Params> {
        Some(self.clone())
    }

    fn decode_response(value: Value) -> json::Result<Self::Response> {
        single(value)
    }
}
