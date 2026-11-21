//! A small host-side BRAVIA remote; see README for usage.

use std::env;
use std::error::Error;
use std::num::NonZeroI32;

use sony_bravia_api::app_control::GetApplicationList;
use sony_bravia_api::app_control::GetApplicationStatusList;
use sony_bravia_api::app_control::SetActiveApp;
use sony_bravia_api::audio::AudioTarget;
use sony_bravia_api::audio::GetSoundSettings;
use sony_bravia_api::audio::GetVolumeInformation;
use sony_bravia_api::audio::SetAudioMute;
use sony_bravia_api::audio::SetAudioVolume;
use sony_bravia_api::audio::Volume;
use sony_bravia_api::av_content::GetCurrentExternalInputsStatus;
use sony_bravia_api::av_content::GetPlayingContentInfo;
use sony_bravia_api::av_content::SetPlayContent;
use sony_bravia_api::system::GetPowerStatus;
use sony_bravia_api::system::GetWolMode;
use sony_bravia_api::system::SetPowerStatus;
use sony_bravia_reqwest::Client;

const USAGE: &str = "usage: tv_remote URL power [on|off] | audio | audio-output | mute on|off | volume speaker|headphone LEVEL | volume-step speaker|headphone SIGNED_STEPS | inputs | input URI | playing | wol | apps | app-status | app URI";
const OUTPUT_TERMINAL: &str = "outputTerminal";

#[derive(Debug)]
enum Command {
    Power(Option<bool>),
    Audio,
    AudioOutput,
    Mute(bool),
    Volume(AudioTarget, Volume),
    Inputs,
    Input(String),
    Playing,
    Wol,
    Apps,
    AppStatus,
    App(String),
}

fn parse_command(args: &[String]) -> Result<Command, &'static str> {
    match args {
        [command] if command == "power" => Ok(Command::Power(None)),
        [command, value] if command == "power" => match value.as_str() {
            "on" => Ok(Command::Power(Some(true))),
            "off" => Ok(Command::Power(Some(false))),
            _ => Err(USAGE),
        },
        [command] if command == "audio" => Ok(Command::Audio),
        [command] if command == "audio-output" => Ok(Command::AudioOutput),
        [command, value] if command == "mute" => match value.as_str() {
            "on" => Ok(Command::Mute(true)),
            "off" => Ok(Command::Mute(false)),
            _ => Err(USAGE),
        },
        [command, target, value] if matches!(command.as_str(), "volume" | "volume-step") => {
            let target = match target.as_str() {
                "speaker" => AudioTarget::Speaker,
                "headphone" => AudioTarget::Headphone,
                _ => return Err(USAGE),
            };
            let volume = if command == "volume" {
                Volume::Absolute(value.parse().map_err(|_| USAGE)?)
            } else {
                if !value.starts_with(['+', '-']) {
                    return Err(USAGE);
                }
                let steps = value.parse::<NonZeroI32>().map_err(|_| USAGE)?;
                if steps.get() > 0 {
                    Volume::Increase(steps.get().unsigned_abs())
                } else {
                    Volume::Decrease(steps.get().unsigned_abs())
                }
            };
            Ok(Command::Volume(target, volume))
        }
        [command] if command == "inputs" => Ok(Command::Inputs),
        [command, uri] if command == "input" && !uri.is_empty() => Ok(Command::Input(uri.clone())),
        [command] if command == "playing" => Ok(Command::Playing),
        [command] if command == "wol" => Ok(Command::Wol),
        [command] if command == "apps" => Ok(Command::Apps),
        [command] if command == "app-status" => Ok(Command::AppStatus),
        [command, uri] if command == "app" && !uri.is_empty() => Ok(Command::App(uri.clone())),
        _ => Err(USAGE),
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    let url = args.next().ok_or(USAGE)?;
    let command = parse_command(&args.collect::<Vec<_>>())?;
    let psk = env::var("BRAVIA_PSK").ok();
    let mut client = Client::new(&url, psk.as_deref())?;
    match command {
        Command::Power(None) => println!("{:?}", client.execute(GetPowerStatus).await?),
        Command::Power(Some(status)) => client.execute(SetPowerStatus { status }).await?,
        Command::Audio => {
            for output in client.execute(GetVolumeInformation).await? {
                println!(
                    "{}: volume {} ({}..{}), muted {}",
                    output.target, output.volume, output.min_volume, output.max_volume, output.mute
                );
            }
        }
        Command::AudioOutput => {
            let settings = client
                .execute(GetSoundSettings {
                    target: Some(OUTPUT_TERMINAL.to_owned()),
                })
                .await?;
            let output = settings
                .into_iter()
                .find(|setting| setting.target == OUTPUT_TERMINAL)
                .ok_or("TV did not report an audio output terminal")?;
            println!("{}", output.current_value);
        }
        Command::Mute(status) => client.execute(SetAudioMute { status }).await?,
        Command::Volume(target, volume) => {
            client.execute(SetAudioVolume { target, volume }).await?
        }
        Command::Inputs => {
            for input in client.execute(GetCurrentExternalInputsStatus).await? {
                println!(
                    "{}: {} ({}, connected {})",
                    input.uri, input.title, input.label, input.connection
                );
            }
        }
        Command::Input(uri) => client.execute(SetPlayContent { uri }).await?,
        Command::Playing => println!("{:?}", client.execute(GetPlayingContentInfo).await?),
        Command::Wol => println!("{:?}", client.execute(GetWolMode).await?),
        Command::Apps => {
            for app in client.execute(GetApplicationList).await? {
                println!("{}: {}", app.title, app.uri);
            }
        }
        Command::AppStatus => {
            for status in client.execute(GetApplicationStatusList).await? {
                println!("{}: {}", status.name, status.status);
            }
        }
        Command::App(uri) => client.execute(SetActiveApp { uri }).await?,
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_owned()).collect()
    }

    #[test]
    fn accepts_explicit_commands_and_rejects_invalid_arguments() {
        assert!(matches!(
            parse_command(&args(&["power"])),
            Ok(Command::Power(None))
        ));
        assert!(matches!(
            parse_command(&args(&["power", "off"])),
            Ok(Command::Power(Some(false)))
        ));
        assert!(matches!(
            parse_command(&args(&["mute", "on"])),
            Ok(Command::Mute(true))
        ));
        assert!(matches!(
            parse_command(&args(&["volume", "speaker", "20"])),
            Ok(Command::Volume(AudioTarget::Speaker, Volume::Absolute(20)))
        ));
        assert!(matches!(
            parse_command(&args(&["volume-step", "speaker", "+1"])),
            Ok(Command::Volume(AudioTarget::Speaker, Volume::Increase(1)))
        ));
        assert!(matches!(
            parse_command(&args(&["volume-step", "headphone", "-1"])),
            Ok(Command::Volume(AudioTarget::Headphone, Volume::Decrease(1)))
        ));
        assert!(matches!(
            parse_command(&args(&["audio-output"])),
            Ok(Command::AudioOutput)
        ));
        assert!(matches!(
            parse_command(&args(&["input", "extInput:hdmi?port=1"])),
            Ok(Command::Input(_))
        ));
        assert!(matches!(parse_command(&args(&["apps"])), Ok(Command::Apps)));
        assert!(matches!(
            parse_command(&args(&["app-status"])),
            Ok(Command::AppStatus)
        ));
        assert!(matches!(
            parse_command(&args(&["app", "app:example"])),
            Ok(Command::App(_))
        ));
        for values in [
            &["power", "toggle"][..],
            &["volume", "speaker", "-1"],
            &["volume-step", "speaker", "1"],
            &["volume-step", "speaker", "+0"],
            &["volume-step", "speaker", "-0"],
            &["mute"],
            &["input", ""],
            &["app", ""],
            &["audio", "extra"],
        ] {
            assert!(parse_command(&args(values)).is_err());
        }
    }
}
