//! Helpers for Sony's fixed-length result arrays.
use de::Error as _;

use super::*;

pub(crate) fn single<T: de::DeserializeOwned>(value: Vec<json::Value>) -> json::Result<T> {
    let [value] = json::from_value::<[T; 1]>(value.into())?;
    Ok(value)
}

pub(crate) fn empty(value: Vec<json::Value>) -> json::Result<()> {
    let [] = json::from_value::<[json::Value; 0]>(value.into())?;
    Ok(())
}

pub(crate) fn zero(value: Vec<json::Value>) -> json::Result<()> {
    let value = single::<i32>(value)?;
    if value == 0 {
        Ok(())
    } else {
        Err(json::Error::custom("expected zero result"))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn check<C: SonyBraviaTvCommand>(
        command: C,
        method: &str,
        version: &str,
        params: json::Value,
        result: json::Value,
    ) -> C::Response {
        let id = NonZeroI32::new(7).unwrap();

        let request = Request::from_command(command, id)
            .unwrap()
            .to_value()
            .unwrap();

        assert_eq!(
            request,
            json::json!({"id":7,"method":method,"version":version,"params":params})
        );

        Response::from_value(json::json!({"id":7,"result":result}))
            .unwrap()
            .response::<C>()
            .unwrap()
            .unwrap()
    }

    #[test]
    fn audio_command_shapes_and_values() {
        let sound = check(
            audio::GetSoundSettings::default(),
            "getSoundSettings",
            "1.1",
            json::json!([{}]),
            json::json!([[{"target":"outputTerminal","currentValue":"speaker"}]]),
        );
        assert_eq!(sound[0].current_value, "speaker");
        let volumes = check(
            audio::GetVolumeInformation,
            "getVolumeInformation",
            "1.0",
            json::json!([]),
            json::json!([[{"target":"speaker","volume":25,"mute":false,"minVolume":0,"maxVolume":100}]]),
        );
        assert_eq!(volumes[0].volume, 25);

        check(
            audio::SetAudioMute { status: true },
            "setAudioMute",
            "1.0",
            json::json!([{"status":true}]),
            json::json!([0]),
        );

        for (volume, expected) in [
            (audio::Volume::Absolute(18), "18"),
            (audio::Volume::Increase(1), "+1"),
            (audio::Volume::Decrease(10), "-10"),
        ] {
            check(
                audio::SetAudioVolume {
                    target: audio::AudioTarget::Speaker,
                    volume,
                },
                "setAudioVolume",
                "1.0",
                json::json!([{"target":"speaker","volume":expected}]),
                json::json!([0]),
            );
        }
        assert!(audio::SetAudioMute::decode_response(vec![json::json!(1)]).is_err());
        assert!(audio::SetAudioVolume::decode_response(vec![]).is_err());

        let settings = check(
            audio::GetSpeakerSettings {
                target: Some("tvPosition".into()),
            },
            "getSpeakerSettings",
            "1.0",
            json::json!([{"target":"tvPosition"}]),
            json::json!([[{"target":"tvPosition","currentValue":"wallMount"}]]),
        );
        assert_eq!(settings[0].current_value, "wallMount");

        let setting = audio::Setting {
            target: "outputTerminal".into(),
            value: "speaker".into(),
        };

        check(
            audio::SetSoundSettings {
                settings: vec![setting],
            },
            "setSoundSettings",
            "1.1",
            json::json!([{"settings":[{"target":"outputTerminal","value":"speaker"}]}]),
            json::json!([]),
        );

        check(
            audio::SetAudioVolumeV1_2 {
                target: audio::AudioTarget::All,
                volume: audio::Volume::Absolute(5),
                ui: Some(audio::VolumeUi::On),
            },
            "setAudioVolume",
            "1.2",
            json::json!([{"target":"","volume":"5","ui":"on"}]),
            json::json!([]),
        );
    }

    #[test]
    fn input_and_application_shapes() {
        let favorites = check(
            av_content::GetFavoriteList,
            "getFavoriteList",
            "1.0",
            json::json!([]),
            json::json!([[{"source":"fav:1","label":"Favorites"}]]),
        );
        assert_eq!(favorites[0].source, "fav:1");
        let inputs_v11 = check(
            av_content::GetCurrentExternalInputsStatusV1_1,
            "getCurrentExternalInputsStatus",
            "1.1",
            json::json!([]),
            json::json!([[{"uri":"extInput:hdmi?port=1","title":"HDMI 1","connection":true,"label":"","status":"active"}]]),
        );
        assert_eq!(inputs_v11[0].status.as_deref(), Some("active"));
        let inputs = check(
            av_content::GetCurrentExternalInputsStatus,
            "getCurrentExternalInputsStatus",
            "1.0",
            json::json!([]),
            json::json!([[{"uri":"extInput:hdmi?port=1","title":"HDMI 1","connection":true,"label":""}]]),
        );
        assert_eq!(inputs[0].uri, "extInput:hdmi?port=1");

        check(
            av_content::SetPlayContent {
                uri: "extInput:hdmi?port=1".into(),
            },
            "setPlayContent",
            "1.0",
            json::json!([{"uri":"extInput:hdmi?port=1"}]),
            json::json!([]),
        );

        let content = check(
            av_content::GetContentList {
                uri: Some("extInput:hdmi".into()),
                start_index: Some(0),
                count: Some(50),
            },
            "getContentList",
            "1.5",
            json::json!([{"uri":"extInput:hdmi","stIdx":0,"cnt":50}]),
            json::json!([[{"uri":"extInput:hdmi?port=1","title":"HDMI 1","index":0}]]),
        );
        assert_eq!(content[0].title.as_deref(), Some("HDMI 1"));

        let apps = check(
            app_control::GetApplicationList,
            "getApplicationList",
            "1.0",
            json::json!([]),
            json::json!([[{"title":"YouTube","uri":"app"}]]),
        );
        assert_eq!(apps[0].icon, "");

        check(
            app_control::SetActiveApp { uri: "app".into() },
            "setActiveApp",
            "1.0",
            json::json!([{"uri":"app"}]),
            json::json!([]),
        );
    }

    #[test]
    fn discovered_screen_status_shapes() {
        let banner = check(
            video_screen::GetBannerMode,
            "getBannerMode",
            "1.0",
            json::json!([]),
            json::json!([{"currentValue":"small"}]),
        );
        assert_eq!(banner.current_value, "small");
        let mode = check(
            video_screen::GetMultiScreenMode,
            "getMultiScreenMode",
            "1.1",
            json::json!([]),
            json::json!([{"mode":"single","option":{"internetTVMode":false}}]),
        );
        assert_eq!(mode.option.unwrap().internet_tv_mode, Some(false));
        let position = check(
            video_screen::GetPipSubScreenPosition,
            "getPipSubScreenPosition",
            "1.0",
            json::json!([]),
            json::json!([{"position":"invalid"}]),
        );
        assert_eq!(position.position, "invalid");
        let audio = check(
            video_screen::GetAudioSourceScreen,
            "getAudioSourceScreen",
            "1.0",
            json::json!([]),
            json::json!([{"screen":"main"}]),
        );
        assert_eq!(audio.screen, "main");
    }

    #[test]
    fn browser_and_cec_command_shapes() {
        check(
            browser::SetTextUrl {
                url: "https://example.org".into(),
                title: None,
                kind: None,
                favicon: None,
            },
            "setTextUrl",
            "1.0",
            json::json!([{"url":"https://example.org"}]),
            json::json!([0]),
        );
        check(
            browser::ActBrowserControl {
                control: "back".into(),
            },
            "actBrowserControl",
            "1.0",
            json::json!([{"control":"back"}]),
            json::json!([0]),
        );
        check(
            cec::SetPowerSyncMode {
                sink_power_off_sync: true,
                source_power_on_sync: false,
            },
            "setPowerSyncMode",
            "1.0",
            json::json!([{"sinkPowerOffSync":true,"sourcePowerOnSync":false}]),
            json::json!([]),
        );
    }

    #[test]
    fn discovery_system_and_video_shapes() {
        let services = check(
            guide::GetSupportedApiInfo::default(),
            "getSupportedApiInfo",
            "1.0",
            json::json!([{}]),
            json::json!([[{"service":"system","protocols":["xhrpost:jsonizer"],"apis":[{"name":"getPowerStatus","versions":[{"version":"1.0"}]}],"notifications":null}]]),
        );
        assert_eq!(services[0].apis[0].name, "getPowerStatus");

        let info = check(
            system::GetSystemInformation,
            "getSystemInformation",
            "1.0",
            json::json!([]),
            json::json!([{"product":"TV","model":"example","name":"BRAVIA"}]),
        );
        assert_eq!(info.generation, "");

        let buttons = check(
            system::GetRemoteControllerInfo,
            "getRemoteControllerInfo",
            "1.0",
            json::json!([]),
            json::json!([{"bundled":true},[{"name":"Home","value":"code"}]]),
        );
        assert_eq!(buttons[0].name, "Home");

        let key = check(
            encryption::GetPublicKey,
            "getPublicKey",
            "1.0",
            json::json!([]),
            json::json!([{"publicKey":"key"}]),
        );
        assert_eq!(key.public_key, "key");

        let scene = check(
            video_screen::GetSceneSetting,
            "getSceneSetting",
            "1.0",
            json::json!([]),
            json::json!([{"currentValue":"auto","candidate":[{"value":"auto"}]}]),
        );
        assert_eq!(scene.candidate.unwrap()[0].value, "auto");

        let picture = check(
            video::GetPictureQualitySettings::default(),
            "getPictureQualitySettings",
            "1.0",
            json::json!([{}]),
            json::json!([[{"target":"color","currentValue":"2","candidate":[{"min":0,"max":100,"step":1}]}]]),
        );
        assert!(picture[0].is_available);
        assert_eq!(picture[0].candidate.as_ref().unwrap()[0].max, Some(100.0));
    }

    #[test]

    fn device_controls_and_scalar_parameters() {
        assert_eq!(audio::SetAudioMute::SERVICE, "audio");
        assert_eq!(av_content::SetPlayContent::SERVICE, "avContent");
        assert_eq!(app_control::SetTextForm::SERVICE, "appControl");
        assert_eq!(guide::GetSupportedApiInfo::SERVICE, "guide");
        assert_eq!(encryption::GetPublicKey::SERVICE, "encryption");
        assert_eq!(system::GetSystemInformation::SERVICE, "system");
        assert_eq!(video::GetScreenRotation::SERVICE, "video");
        assert_eq!(video_screen::GetSceneSetting::SERVICE, "videoScreen");

        let clock = check(
            system::GetCurrentTimeV1_1,
            "getCurrentTime",
            "1.1",
            json::json!([]),
            json::json!([{"dateTime":"2018-10-03T13:03:59+0100","timeZoneOffsetMinute":60,"dstOffsetMinute":60}]),
        );
        assert_eq!(clock.time_zone_offset_minute, 60);

        let network = check(
            system::GetNetworkSettings::default(),
            "getNetworkSettings",
            "1.0",
            json::json!([{}]),
            json::json!([[{"netif":"eth0","hwAddr":"00","ipAddrV4":"1.2.3.4","ipAddrV6":"::1","netmask":"255.255.255.0","gateway":"1.2.3.1","dns":[]}]]),
        );
        assert_eq!(network[0].ip_addr_v4, "1.2.3.4");

        let remote = check(
            system::GetRemoteDeviceSettings::default(),
            "getRemoteDeviceSettings",
            "1.0",
            json::json!([{}]),
            json::json!([[{"target":"accessPermission","currentValue":"on"}]]),
        );
        assert_eq!(remote[0].current_value, "on");

        check(
            app_control::SetTextForm {
                text: "hello world!!".into(),
            },
            "setTextForm",
            "1.0",
            json::json!(["hello world!!"]),
            json::json!([0]),
        );

        check(
            video_screen::SetSceneSetting {
                value: "auto".into(),
            },
            "setSceneSetting",
            "1.0",
            json::json!([{"value":"auto"}]),
            json::json!([]),
        );

        check(
            video::SetScreenRotation { rotation: 90 },
            "setScreenRotation",
            "1.0",
            json::json!([{"rotation":90}]),
            json::json!([]),
        );

        check(
            system::SetWolMode { enabled: true },
            "setWolMode",
            "1.0",
            json::json!([{"enabled":true}]),
            json::json!([]),
        );
    }
}
