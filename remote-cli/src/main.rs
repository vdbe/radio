use cli::Cli;
use config::read_config;
use error::Result;

mod cli;
mod config;
mod error;

#[tokio::main]
async fn main() -> Result<()> {
    let config = read_config()?;

    let args = Cli::default();
    args.command
        .execute(&config.connection.host, config.connection.pin)
        .await?;

    Ok(())
}
