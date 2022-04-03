use dotenv::dotenv;
use std::env;

use fsapi::FsApi;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let host = env::var("HOST")?;
    let pin = env::var("PIN")?.parse()?;

    let session_id = FsApi::create_session(&host, pin).await?;

    loop {
        if let Some(notifications) = FsApi::get_notifications(session_id, &host, pin).await? {
            for notification in notifications {
                println!("{}: {}", notification.node, notification.value);
            }
        }
    }
}
