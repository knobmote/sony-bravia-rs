use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Request {
    id: NonZeroI32,
    method: &'static str,
    params: Vec<json::Value>,
    version: &'static str,
}

impl Request {
    pub fn from_command<T>(command: T, id: NonZeroI32) -> json::Result<Self>
    where
        T: SonyBraviaTvCommand,
    {
        let params = command
            .params()
            .map(json::to_value)
            .transpose()?
            .map_or_default(|params| vec![params]);
        let request = Self {
            id,
            method: T::METHOD,
            params,
            version: T::VERSION,
        };
        Ok(request)
    }

    pub fn to_value(self) -> json::Result<json::Value> {
        json::to_value(self)
    }

    pub fn to_json(&self) -> json::Result<String> {
        json::to_string(self)
    }
}

// A result takes precedence even when an error field is present. If a result is
// present but invalid, do not silently reinterpret the response as an error.
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Success {
        id: NonZeroI32,
        result: Vec<json::Value>,
    },
    Failure {
        id: NonZeroI32,
        error: Error,
        #[serde(default, rename = "result", deserialize_with = "reject_result")]
        _result: (),
    },
}

fn reject_result<'de, D: serde::Deserializer<'de>>(_deserializer: D) -> Result<(), D::Error> {
    Err(de::Error::custom("unexpected result in error response"))
}

impl Response {
    pub fn from_value(value: json::Value) -> json::Result<Self> {
        json::from_value(value)
    }

    pub fn from_json(text: &str) -> json::Result<Self> {
        json::from_str(text)
    }

    pub fn id(&self) -> NonZeroI32 {
        match self {
            Self::Success { id, .. } => *id,
            Self::Failure { id, .. } => *id,
        }
    }

    pub fn response<T>(self) -> json::Result<Result<T::Response, Error>>
    where
        T: SonyBraviaTvCommand,
    {
        let response = match self {
            Self::Success { result, .. } => Ok(T::decode_response(result)?),
            Self::Failure { error, .. } => Err(error),
        };
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCommand;

    impl SonyBraviaTvCommand for TestCommand {
        const SERVICE: &str = "test";
        const METHOD: &str = "test";
        const VERSION: &str = "1.0";

        type Params = ();
        type Response = Vec<i32>;

        fn params(&self) -> Option<Self::Params> {
            None
        }

        fn decode_response(result: Vec<json::Value>) -> json::Result<Self::Response> {
            json::from_value(result.into())
        }
    }

    #[test]
    fn deserializes_error_responses() {
        for (code, expected) in [
            (401, ErrorCode::Unauthorized),
            (418, ErrorCode::Unknown(418)),
        ] {
            // let response =  Response::from<Vec<json::Value>> =
            let response =
                Response::from_json(&format!(r#"{{"error":[{code},"TV message"],"id":2}}"#))
                    .unwrap();
            assert_eq!(response.id().get(), 2);
            let response = response.response::<TestCommand>().unwrap();
            let error = response.unwrap_err();
            assert_eq!(error.code, expected);
            assert_eq!(error.message, "TV message");
        }
    }

    #[test]
    fn deserializes_success_and_ignores_error() {
        let response = Response::from_json(r#"{"result":[42],"error":"ignored","id":2}"#).unwrap();
        assert_eq!(response.id().get(), 2);
        let response = response.response::<TestCommand>().unwrap();
        assert_eq!(response.unwrap(), vec![42]);
    }

    #[test]
    fn rejects_invalid_response_payloads() {
        for input in [
            r#"{"error":[401],"id":2}"#,
            r#"{"error":[401,"bad",3],"id":2}"#,
            r#"{"error":[401,"bad"],"id":0}"#,
            r#"{"id":2}"#,
            r#"{"result":"invalid","error":[401,"bad"],"id":2}"#,
        ] {
            assert!(
                Response::from_json(input).is_err(),
                // json::from_str::<SonyBraviaTvResponse<Vec<i32>>>(input).is_err(),
                "{input}"
            );
        }
    }
}
