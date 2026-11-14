//! Commands for the Sony system service.

use super::*;

/// Get the display's current power state.
#[derive(Clone, Copy, Debug, Default)]
pub struct GetPowerStatus;

/// A display's reported power state.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PowerStatus {
    /// The display is on.
    Active,
    /// The display is in standby.
    Standby,
}

#[derive(Deserialize)]
struct PowerStatusResult {
    status: PowerStatus,
}

impl SonyBraviaTvCommand for GetPowerStatus {
    const SERVICE: &str = "system";
    const METHOD: &str = "getPowerStatus";
    const VERSION: &str = "1.0";

    type Params = ();
    type Response = PowerStatus;

    fn params(&self) -> Option<Self::Params> {
        None
    }

    fn decode_response(result: json::Value) -> json::Result<Self::Response> {
        let [result]: [PowerStatusResult; 1] = json::from_value(result)?;
        Ok(result.status)
    }
}

/// Set the display's power state. `true` requests on; `false` requests standby.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct SetPowerStatus {
    pub status: bool,
}

impl SonyBraviaTvCommand for SetPowerStatus {
    const SERVICE: &str = "system";
    const METHOD: &str = "setPowerStatus";
    const VERSION: &str = "1.0";

    type Params = Self;
    type Response = ();

    fn params(&self) -> Option<Self::Params> {
        Some(*self)
    }

    fn decode_response(result: json::Value) -> json::Result<Self::Response> {
        let []: [json::Value; 0] = json::from_value(result)?;
        Ok(())
    }
}

/// Read the display's device information.
#[derive(Clone, Copy, Debug, Default)]
pub struct GetSystemInformation;

/// General display information. Missing optional strings use Sony's empty default.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SystemInformation {
    pub product: String,
    pub model: String,
    pub name: String,
    #[serde(default)]
    pub language: String,
    #[serde(default)]
    pub serial: String,
    #[serde(default)]
    pub mac_addr: String,
    #[serde(default)]
    pub generation: String,
}

impl SonyBraviaTvCommand for GetSystemInformation {
    const SERVICE: &str = "system";
    const METHOD: &str = "getSystemInformation";
    const VERSION: &str = "1.0";

    type Params = ();
    type Response = SystemInformation;

    fn params(&self) -> Option<Self::Params> {
        None
    }

    fn decode_response(value: Value) -> json::Result<Self::Response> {
        single(value)
    }
}

/// Read the display's current clock as an ISO 8601 string.
#[derive(Clone, Copy, Debug, Default)]
pub struct GetCurrentTime;

impl SonyBraviaTvCommand for GetCurrentTime {
    const SERVICE: &str = "system";
    const METHOD: &str = "getCurrentTime";
    const VERSION: &str = "1.0";

    type Params = ();
    type Response = String;

    fn params(&self) -> Option<Self::Params> {
        None
    }

    fn decode_response(value: Value) -> json::Result<Self::Response> {
        single(value)
    }
}

/// Read whether the display accepts Wake-on-LAN packets.
#[derive(Clone, Copy, Debug, Default)]
pub struct GetWolMode;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct WolMode {
    pub enabled: bool,
}

impl SonyBraviaTvCommand for GetWolMode {
    const SERVICE: &str = "system";
    const METHOD: &str = "getWolMode";
    const VERSION: &str = "1.0";

    type Params = ();
    type Response = WolMode;

    fn params(&self) -> Option<Self::Params> {
        None
    }

    fn decode_response(value: Value) -> json::Result<Self::Response> {
        single(value)
    }
}

/// Enable or disable Wake-on-LAN.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct SetWolMode {
    pub enabled: bool,
}

impl SonyBraviaTvCommand for SetWolMode {
    const SERVICE: &str = "system";
    const METHOD: &str = "setWolMode";
    const VERSION: &str = "1.0";

    type Params = Self;
    type Response = ();

    fn params(&self) -> Option<Self::Params> {
        Some(*self)
    }

    fn decode_response(value: Value) -> json::Result<Self::Response> {
        empty(value)
    }
}

/// Request a display reboot.
#[derive(Clone, Copy, Debug, Default)]
pub struct RequestReboot;

impl SonyBraviaTvCommand for RequestReboot {
    const SERVICE: &str = "system";
    const METHOD: &str = "requestReboot";
    const VERSION: &str = "1.0";

    type Params = ();
    type Response = ();

    fn params(&self) -> Option<Self::Params> {
        None
    }

    fn decode_response(value: Value) -> json::Result<Self::Response> {
        empty(value)
    }
}

/// Read the REST interface's identification and version.
#[derive(Clone, Copy, Debug, Default)]
pub struct GetInterfaceInformation;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InterfaceInformation {
    pub product_category: String,
    pub product_name: String,
    pub model_name: String,
    pub server_name: String,
    pub interface_version: String,
}

impl SonyBraviaTvCommand for GetInterfaceInformation {
    const SERVICE: &str = "system";
    const METHOD: &str = "getInterfaceInformation";
    const VERSION: &str = "1.0";

    type Params = ();
    type Response = InterfaceInformation;

    fn params(&self) -> Option<Self::Params> {
        None
    }

    fn decode_response(value: Value) -> json::Result<Self::Response> {
        single(value)
    }
}

/// Retrieve the remote control button names and IRCC codes.
#[derive(Clone, Copy, Debug, Default)]
pub struct GetRemoteControllerInfo;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct RemoteButton {
    pub name: String,
    pub value: String,
}

impl SonyBraviaTvCommand for GetRemoteControllerInfo {
    const SERVICE: &str = "system";
    const METHOD: &str = "getRemoteControllerInfo";
    const VERSION: &str = "1.0";

    type Params = ();
    type Response = Vec<RemoteButton>;

    fn params(&self) -> Option<Self::Params> {
        None
    }

    fn decode_response(result: Value) -> json::Result<Self::Response> {
        let (_, buttons): (serde::de::IgnoredAny, Vec<RemoteButton>) = json::from_value(result)?;
        Ok(buttons)
    }
}

/// List supported system capabilities, including Wake-on-LAN information.
#[derive(Clone, Copy, Debug, Default)]
pub struct GetSystemSupportedFunction;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct SystemFunction {
    pub option: String,
    pub value: String,
}

impl SonyBraviaTvCommand for GetSystemSupportedFunction {
    const SERVICE: &str = "system";
    const METHOD: &str = "getSystemSupportedFunction";
    const VERSION: &str = "1.0";

    type Params = ();
    type Response = Vec<SystemFunction>;

    fn params(&self) -> Option<Self::Params> {
        None
    }

    fn decode_response(value: Value) -> json::Result<Self::Response> {
        single(value)
    }
}

/// Get the current power-saving setting.
#[derive(Clone, Copy, Debug, Default)]
pub struct GetPowerSavingMode;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct PowerSavingMode {
    pub mode: String,
}

impl SonyBraviaTvCommand for GetPowerSavingMode {
    const SERVICE: &str = "system";
    const METHOD: &str = "getPowerSavingMode";
    const VERSION: &str = "1.0";

    type Params = ();
    type Response = PowerSavingMode;

    fn params(&self) -> Option<Self::Params> {
        None
    }

    fn decode_response(value: Value) -> json::Result<Self::Response> {
        single(value)
    }
}

/// Change the power-saving setting.
#[derive(Clone, Debug, Serialize)]
pub struct SetPowerSavingMode {
    pub mode: String,
}

impl SonyBraviaTvCommand for SetPowerSavingMode {
    const SERVICE: &str = "system";
    const METHOD: &str = "setPowerSavingMode";
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

/// Read the LED indicator mode and status.
#[derive(Clone, Copy, Debug, Default)]
pub struct GetLedIndicatorStatus;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct LedIndicatorStatus {
    pub mode: String,
    pub status: Option<String>,
}

impl SonyBraviaTvCommand for GetLedIndicatorStatus {
    const SERVICE: &str = "system";
    const METHOD: &str = "getLEDIndicatorStatus";
    const VERSION: &str = "1.0";

    type Params = ();
    type Response = LedIndicatorStatus;

    fn params(&self) -> Option<Self::Params> {
        None
    }

    fn decode_response(value: Value) -> json::Result<Self::Response> {
        single(value)
    }
}

/// Change the LED indicator mode and optionally its status.
#[derive(Clone, Debug, Serialize)]
pub struct SetLedIndicatorStatus {
    pub mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl SonyBraviaTvCommand for SetLedIndicatorStatus {
    const SERVICE: &str = "system";
    const METHOD: &str = "setLEDIndicatorStatus";
    const VERSION: &str = "1.1";

    type Params = Self;
    type Response = ();

    fn params(&self) -> Option<Self::Params> {
        Some(self.clone())
    }

    fn decode_response(value: Value) -> json::Result<Self::Response> {
        empty(value)
    }
}

/// Get the current time with time zone and daylight-saving offsets.
#[derive(Clone, Copy, Debug, Default)]
pub struct GetCurrentTimeV1_1;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CurrentTime {
    pub date_time: String,
    pub time_zone_offset_minute: i32,
    pub dst_offset_minute: i32,
}

impl SonyBraviaTvCommand for GetCurrentTimeV1_1 {
    const SERVICE: &str = "system";
    const METHOD: &str = "getCurrentTime";
    const VERSION: &str = "1.1";

    type Params = ();
    type Response = CurrentTime;

    fn params(&self) -> Option<Self::Params> {
        None
    }

    fn decode_response(value: Value) -> json::Result<Self::Response> {
        single(value)
    }
}

/// Read network settings for one interface or all interfaces.
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetNetworkSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netif: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NetworkSettings {
    pub netif: String,
    pub hw_addr: String,
    #[serde(rename = "ipAddrV4")]
    pub ip_addr_v4: String,
    #[serde(rename = "ipAddrV6")]
    pub ip_addr_v6: String,
    pub netmask: String,
    pub gateway: String,
    pub dns: Vec<String>,
}

impl SonyBraviaTvCommand for GetNetworkSettings {
    const SERVICE: &str = "system";
    const METHOD: &str = "getNetworkSettings";
    const VERSION: &str = "1.0";

    type Params = Self;
    type Response = Vec<NetworkSettings>;

    fn params(&self) -> Option<Self::Params> {
        Some(self.clone())
    }

    fn decode_response(value: Value) -> json::Result<Self::Response> {
        single(value)
    }
}

/// Read remote-device access settings.
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetRemoteDeviceSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RemoteDeviceSetting {
    pub target: String,
    pub current_value: String,
}

impl SonyBraviaTvCommand for GetRemoteDeviceSettings {
    const SERVICE: &str = "system";
    const METHOD: &str = "getRemoteDeviceSettings";
    const VERSION: &str = "1.0";

    type Params = Self;
    type Response = Vec<RemoteDeviceSetting>;

    fn params(&self) -> Option<Self::Params> {
        Some(self.clone())
    }

    fn decode_response(value: Value) -> json::Result<Self::Response> {
        single(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_requests_match_sony_examples() {
        let id = NonZeroI32::new(50).unwrap();
        assert_eq!(GetPowerStatus::SERVICE, "system");
        assert_eq!(
            Request::from_command(GetPowerStatus, id)
                .unwrap()
                .to_value()
                .unwrap(),
            json::json!({"method":"getPowerStatus", "id":50, "params":[], "version":"1.0"})
        );

        for status in [true, false] {
            assert_eq!(
                Request::from_command(SetPowerStatus { status }, id)
                    .unwrap()
                    .to_value()
                    .unwrap(),
                json::json!({"method":"setPowerStatus", "id":50, "params":[{"status":status}], "version":"1.0"})
            );
        }
    }

    #[test]
    fn power_responses_are_typed_and_validate_the_result_shape() {
        for (status, expected) in [
            ("active", PowerStatus::Active),
            ("standby", PowerStatus::Standby),
        ] {
            let response = Response::from_json(&format!(
                r#"{{"id":50,"result":[{{"status":"{status}"}}]}}"#
            ))
            .unwrap();
            assert_eq!(
                response.response::<GetPowerStatus>().unwrap().unwrap(),
                expected
            );
        }

        for result in [
            r#"[]"#,
            r#"[{"status":"active"},{"status":"standby"}]"#,
            r#"[{"status":"unknown"}]"#,
        ] {
            let response =
                Response::from_json(&format!(r#"{{"id":50,"result":{result}}}"#)).unwrap();
            assert!(response.response::<GetPowerStatus>().is_err());
        }
        let response = Response::from_json(r#"{"id":50,"result":[]}"#).unwrap();
        assert!(response.response::<SetPowerStatus>().unwrap().is_ok());
        let response = Response::from_json(r#"{"id":50,"result":[0]}"#).unwrap();
        assert!(response.response::<SetPowerStatus>().is_err());
        let response = Response::from_json(r#"{"id":50,"error":[7,"not available"]}"#).unwrap();
        let error = response.response::<GetPowerStatus>().unwrap().unwrap_err();
        assert_eq!(error.code, ErrorCode::IllegalState);
    }
}
