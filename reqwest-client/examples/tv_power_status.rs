//! Read a BRAVIA TV's power status over HTTP from a desktop computer.

use std::env;
use std::error::Error;

use sony_bravia_api::system::GetPowerStatus;
use sony_bravia_reqwest::Client;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    let base_url = args
        .next()
        .ok_or("usage: tv_power_status http://TV_ADDRESS[:PORT]")?;
    if args.next().is_some() {
        return Err("usage: tv_power_status http://TV_ADDRESS[:PORT]".into());
    }
    let psk = env::var("BRAVIA_PSK").ok();
    let mut client = Client::new(&base_url, psk.as_deref())?;
    let status = client.execute(GetPowerStatus).await?;
    println!("Power status: {status:?}");
    Ok(())
}
