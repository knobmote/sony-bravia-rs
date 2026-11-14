//! Exercise read-only BRAVIA commands against a real TV.

use std::env;
use std::error::Error;

use sony_bravia_api::SonyBraviaTvCommand;
use sony_bravia_api::app_control::GetApplicationList;
use sony_bravia_api::app_control::GetApplicationStatusList;
use sony_bravia_api::audio::GetSpeakerSettings;
use sony_bravia_api::audio::GetVolumeInformation;
use sony_bravia_api::av_content::GetCurrentExternalInputsStatus;
use sony_bravia_api::av_content::GetPlayingContentInfo;
use sony_bravia_api::av_content::GetSchemeList;
use sony_bravia_api::system::GetCurrentTime;
use sony_bravia_api::system::GetCurrentTimeV1_1;
use sony_bravia_api::system::GetInterfaceInformation;
use sony_bravia_api::system::GetLedIndicatorStatus;
use sony_bravia_api::system::GetPowerSavingMode;
use sony_bravia_api::system::GetPowerStatus;
use sony_bravia_api::system::GetRemoteControllerInfo;
use sony_bravia_api::system::GetSystemInformation;
use sony_bravia_api::system::GetSystemSupportedFunction;
use sony_bravia_api::system::GetWolMode;
use sony_bravia_api::video::GetScreenRotation;
use sony_bravia_api::video_screen::GetSceneSetting;
use sony_bravia_client::Error as ProtocolError;
use sony_bravia_reqwest::Client;
use sony_bravia_reqwest::Error as ClientError;

async fn check<C: SonyBraviaTvCommand>(client: &mut Client, command: C) -> bool {
    match client.execute(command).await {
        Ok(_) => {
            println!("PASS {}/{} {}", C::SERVICE, C::METHOD, C::VERSION);
            true
        }
        Err(error) => {
            println!("FAIL {}/{} {}: {error}", C::SERVICE, C::METHOD, C::VERSION);
            match &error {
                ClientError::Protocol(ProtocolError::Device(error))
                | ClientError::Protocol(ProtocolError::Http {
                    device_error: Some(error),
                    ..
                }) => println!("  Sony error code: {:?}", error.code),
                _ => {}
            }
            false
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    let url = args
        .next()
        .ok_or("usage: tv_validate http://TV_ADDRESS[:PORT]")?;
    if args.next().is_some() {
        return Err("usage: tv_validate http://TV_ADDRESS[:PORT]".into());
    }
    let psk = env::var("BRAVIA_PSK").ok();
    let mut client = Client::new(&url, psk.as_deref())?;
    let results = [
        check(&mut client, GetPowerStatus).await,
        check(&mut client, GetSystemInformation).await,
        check(&mut client, GetCurrentTime).await,
        check(&mut client, GetCurrentTimeV1_1).await,
        check(&mut client, GetWolMode).await,
        check(&mut client, GetInterfaceInformation).await,
        check(&mut client, GetRemoteControllerInfo).await,
        check(&mut client, GetSystemSupportedFunction).await,
        check(&mut client, GetPowerSavingMode).await,
        check(&mut client, GetLedIndicatorStatus).await,
        check(&mut client, GetVolumeInformation).await,
        check(&mut client, GetSpeakerSettings::default()).await,
        check(&mut client, GetCurrentExternalInputsStatus).await,
        check(&mut client, GetPlayingContentInfo).await,
        check(&mut client, GetSchemeList).await,
        check(&mut client, GetApplicationList).await,
        check(&mut client, GetApplicationStatusList).await,
        check(&mut client, GetScreenRotation).await,
        check(&mut client, GetSceneSetting).await,
    ];
    let passed = results.iter().filter(|passed| **passed).count();
    println!("{passed}/{} commands passed", results.len());
    if passed != results.len() {
        return Err("one or more commands failed; support varies by TV and power state".into());
    }
    Ok(())
}
