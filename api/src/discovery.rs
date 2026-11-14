//! Methods common to Sony REST services.

use super::*;

/// A service for which common methods can be queried.
pub trait Service {
    const NAME: &'static str;
}

/// Service markers for common discovery requests.
pub mod service {
    use super::*;

    #[derive(Debug)]
    pub struct AccessControl;
    impl Service for AccessControl {
        const NAME: &'static str = "accessControl";
    }
    #[derive(Debug)]
    pub struct AppControl;
    impl Service for AppControl {
        const NAME: &'static str = "appControl";
    }
    #[derive(Debug)]
    pub struct Audio;
    impl Service for Audio {
        const NAME: &'static str = "audio";
    }
    #[derive(Debug)]
    pub struct AvContent;
    impl Service for AvContent {
        const NAME: &'static str = "avContent";
    }
    #[derive(Debug)]
    pub struct Browser;
    impl Service for Browser {
        const NAME: &'static str = "browser";
    }
    #[derive(Debug)]
    pub struct Cec;
    impl Service for Cec {
        const NAME: &'static str = "cec";
    }
    #[derive(Debug)]
    pub struct Encryption;
    impl Service for Encryption {
        const NAME: &'static str = "encryption";
    }
    #[derive(Debug)]
    pub struct Guide;
    impl Service for Guide {
        const NAME: &'static str = "guide";
    }
    #[derive(Debug)]
    pub struct System;
    impl Service for System {
        const NAME: &'static str = "system";
    }
    #[derive(Debug)]
    pub struct Video;
    impl Service for Video {
        const NAME: &'static str = "video";
    }
    #[derive(Debug)]
    pub struct VideoScreen;
    impl Service for VideoScreen {
        const NAME: &'static str = "videoScreen";
    }
}

/// Get the service's supported versions.
#[derive(Debug)]
pub struct GetVersions<S>(PhantomData<S>);

impl<S> GetVersions<S> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<S> Default for GetVersions<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: Service> SonyBraviaTvCommand for GetVersions<S> {
    const SERVICE: &str = S::NAME;
    const METHOD: &str = "getVersions";
    const VERSION: &str = "1.0";
    type Params = ();
    type Response = Vec<String>;
    fn params(&self) -> Option<Self::Params> {
        None
    }
    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        single(result)
    }
}

/// Get method signatures for a service; an empty filter requests all versions.
#[derive(Debug)]
pub struct GetMethodTypes<S> {
    pub version_filter: String,
    service: PhantomData<S>,
}

impl<S> GetMethodTypes<S> {
    pub fn new(version_filter: impl Into<String>) -> Self {
        Self {
            version_filter: version_filter.into(),
            service: PhantomData,
        }
    }
}

/// A method signature as advertised by a Sony service.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(from = "(String, Vec<String>, Vec<String>, String)")]
pub struct MethodType {
    pub name: String,
    pub parameters: Vec<String>,
    pub result: Vec<String>,
    pub version: String,
}

impl From<(String, Vec<String>, Vec<String>, String)> for MethodType {
    fn from(
        (name, parameters, result, version): (String, Vec<String>, Vec<String>, String),
    ) -> Self {
        Self {
            name,
            parameters,
            result,
            version,
        }
    }
}

impl<S: Service> SonyBraviaTvCommand for GetMethodTypes<S> {
    const SERVICE: &str = S::NAME;
    const METHOD: &str = "getMethodTypes";
    const VERSION: &str = "1.0";
    type Params = String;
    type Response = Vec<MethodType>;
    fn params(&self) -> Option<Self::Params> {
        Some(self.version_filter.clone())
    }
    fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
        json::from_value(result.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_methods_use_service_and_decode_discovery_envelopes() {
        let id = NonZeroI32::new(7).unwrap();
        let request = Request::from_command(GetMethodTypes::<service::Audio>::new(""), id)
            .unwrap()
            .to_value()
            .unwrap();
        assert_eq!(
            request,
            json::json!({"id":7,"method":"getMethodTypes","params":[""],"version":"1.0"})
        );
        assert_eq!(GetMethodTypes::<service::Audio>::SERVICE, "audio");
        let response = Response::from_json(
            r#"{"id":7,"results":[["getVolumeInformation",[],["object*"],"1.0"]]}"#,
        )
        .unwrap();
        let methods = response
            .response::<GetMethodTypes<service::Audio>>()
            .unwrap()
            .unwrap();
        assert_eq!(methods[0].name, "getVolumeInformation");
        assert_eq!(methods[0].result, ["object*"]);
        let response = Response::from_json(r#"{"id":7,"result":[["1.0","1.1"]]}"#).unwrap();
        assert_eq!(
            response
                .response::<GetVersions<service::Audio>>()
                .unwrap()
                .unwrap(),
            ["1.0", "1.1"]
        );
    }
}
