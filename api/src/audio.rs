//! Audio volume and mute commands.

use super::*;

/// Read the volume and mute state for each output.
#[derive(Clone, Copy, Debug, Default)]
pub struct GetVolumeInformation;

/// Volume information for one output.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VolumeInformation {
    pub target: String,
    pub volume: i32,
    pub mute: bool,
    pub min_volume: i32,
    pub max_volume: i32,
}

impl SonyBraviaTvCommand for GetVolumeInformation {
    const SERVICE: &str = "audio";
    const METHOD: &str = "getVolumeInformation";
    const VERSION: &str = "1.0";

    type Params = ();
    type Response = Vec<VolumeInformation>;

    fn params(&self) -> Option<Self::Params> {
        None
    }

    fn decode_response(value: Vec<json::Value>) -> json::Result<Self::Response> {
        single(value)
    }
}

/// Change mute state for the display.
#[derive(Clone, Copy, Debug, Serialize)]

pub struct SetAudioMute {
    pub status: bool,
}

impl SonyBraviaTvCommand for SetAudioMute {
    const SERVICE: &str = "audio";
    const METHOD: &str = "setAudioMute";
    const VERSION: &str = "1.0";

    type Params = Self;
    type Response = ();

    fn params(&self) -> Option<Self::Params> {
        Some(*self)
    }

    fn decode_response(value: Vec<json::Value>) -> json::Result<Self::Response> {
        zero(value)
    }
}

/// Set an absolute volume or adjust it by a signed amount.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Volume {
    Absolute(u32),
    Increase(u32),
    Decrease(u32),
}

impl Serialize for Volume {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let value = match self {
            Self::Absolute(n) => n.to_string(),
            Self::Increase(n) => format!("+{n}"),
            Self::Decrease(n) => format!("-{n}"),
        };
        serializer.serialize_str(&value)
    }
}

/// Audio output to control.
#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]

pub enum AudioTarget {
    #[serde(rename = "")]
    All,
    #[serde(rename = "speaker")]
    Speaker,
    #[serde(rename = "headphone")]
    Headphone,
}

/// Change volume using the widely supported version 1.0 API.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct SetAudioVolume {
    pub target: AudioTarget,
    pub volume: Volume,
}

impl SonyBraviaTvCommand for SetAudioVolume {
    const SERVICE: &str = "audio";
    const METHOD: &str = "setAudioVolume";
    const VERSION: &str = "1.0";

    type Params = Self;
    type Response = ();

    fn params(&self) -> Option<Self::Params> {
        Some(*self)
    }

    fn decode_response(value: Vec<json::Value>) -> json::Result<Self::Response> {
        zero(value)
    }
}

/// Get speaker settings, optionally filtering by a device-specific target.
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetSpeakerSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SpeakerSetting {
    pub target: String,
    pub current_value: String,
}

impl SonyBraviaTvCommand for GetSpeakerSettings {
    const SERVICE: &str = "audio";
    const METHOD: &str = "getSpeakerSettings";
    const VERSION: &str = "1.0";

    type Params = Self;
    type Response = Vec<SpeakerSetting>;

    fn params(&self) -> Option<Self::Params> {
        Some(self.clone())
    }

    fn decode_response(value: Vec<json::Value>) -> json::Result<Self::Response> {
        single(value)
    }
}

/// A device-specific setting and the value to apply.
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct Setting {
    pub target: String,
    pub value: String,
}

/// Change one or more speaker settings.
#[derive(Clone, Debug, Serialize)]
pub struct SetSpeakerSettings {
    pub settings: Vec<Setting>,
}

impl SonyBraviaTvCommand for SetSpeakerSettings {
    const SERVICE: &str = "audio";
    const METHOD: &str = "setSpeakerSettings";
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

/// Change one or more sound settings.
#[derive(Clone, Debug, Serialize)]
pub struct SetSoundSettings {
    pub settings: Vec<Setting>,
}

impl SonyBraviaTvCommand for SetSoundSettings {
    const SERVICE: &str = "audio";
    const METHOD: &str = "setSoundSettings";
    const VERSION: &str = "1.1";

    type Params = Self;
    type Response = ();

    fn params(&self) -> Option<Self::Params> {
        Some(self.clone())
    }

    fn decode_response(value: Vec<json::Value>) -> json::Result<Self::Response> {
        empty(value)
    }
}

/// Set volume with explicit control of the display's volume UI (API v1.2).
#[derive(Clone, Debug, Serialize)]
pub struct SetAudioVolumeV1_2 {
    pub target: AudioTarget,
    pub volume: Volume,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui: Option<VolumeUi>,
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum VolumeUi {
    On,
    Off,
}

impl SonyBraviaTvCommand for SetAudioVolumeV1_2 {
    const SERVICE: &str = "audio";
    const METHOD: &str = "setAudioVolume";
    const VERSION: &str = "1.2";

    type Params = Self;
    type Response = ();

    fn params(&self) -> Option<Self::Params> {
        Some(self.clone())
    }

    fn decode_response(value: Vec<json::Value>) -> json::Result<Self::Response> {
        empty(value)
    }
}
