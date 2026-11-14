//! Device registration command advertised by BRAVIA TVs.

use super::*;

/// Register a remote device using the TV's two-parameter registration request.
///
/// PIN-based pairing may require HTTP authentication and a response cookie.
/// The caller must handle those transport headers; the reqwest client currently
/// supports pre-shared-key authentication only. This command has not been
/// validated against the TV because registration changes its device list.
#[derive(Clone, Debug)]
pub struct ActRegister {
    pub client: RegistrationClient,
    pub functions: Vec<RegistrationFunction>,
}

#[derive(Clone, Debug, Serialize)]
pub struct RegistrationClient {
    #[serde(rename = "clientid")]
    pub client_id: String,
    pub nickname: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct RegistrationFunction {
    pub function: String,
    pub value: String,
}

impl SonyBraviaTvCommand for ActRegister {
    const SERVICE: &str = "accessControl";
    const METHOD: &str = "actRegister";
    const VERSION: &str = "1.0";
    type Params = (RegistrationClient, Vec<RegistrationFunction>);
    type Response = ();
    fn params(&self) -> Option<Self::Params> {
        Some((self.client.clone(), self.functions.clone()))
    }
    fn encode_params(&self) -> json::Result<Vec<json::Value>> {
        Ok(vec![
            json::to_value(&self.client)?,
            json::to_value(&self.functions)?,
        ])
    }
    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        empty(result)
    }
}
