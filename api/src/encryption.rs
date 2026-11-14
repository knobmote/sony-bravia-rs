//! Encryption service commands.

use super::*;

/// Retrieve the device's RSA public key.
#[derive(Clone, Copy, Debug, Default)]
pub struct GetPublicKey;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PublicKey {
    pub public_key: String,
}

impl SonyBraviaTvCommand for GetPublicKey {
    const SERVICE: &str = "encryption";
    const METHOD: &str = "getPublicKey";
    const VERSION: &str = "1.0";

    type Params = ();
    type Response = PublicKey;

    fn params(&self) -> Option<Self::Params> {
        None
    }

    fn decode_response(value: Value) -> json::Result<Self::Response> {
        single(value)
    }
}
