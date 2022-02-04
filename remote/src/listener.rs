use dotenv::dotenv;
use std::env;

use fsapi::FsApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let host = env::var("HOST")?;
    let pin = env::var("PIN")?;

    let session_id = FsApi::create_session(&host, &pin).await?;

    loop {
        for notification in FsApi::get_notifications(session_id, &host, &pin).await? {
            println!("{}: {}", notification.node, notification.value);
        }
    }
}
