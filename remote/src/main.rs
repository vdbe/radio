use dotenv::dotenv;
use std::env;

use radio::Radio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let host = env::var("HOST")?;
    let pin = env::var("PIN")?;

    let mut radio = Radio::new(host, pin).await?;

    dbg!(&radio);
    loop {
        radio.update_state().await?;
        dbg!(&radio);
    }

    Ok(())
}
