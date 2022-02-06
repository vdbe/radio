use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use tokio::task;

use radio::Error;
use radio::Radio;

async fn keep_up_to_date(radio: Arc<Radio>) -> Result<(), Error> {
    loop {
        //let notifications = Radio::get_notifications(radio.clone()).await?;
        if let Some(notifications) = radio.get_notifications().await? {
            //let mut tasks = Vec::new();
            for notication in notifications {
                //let radio = radio.clone();
                //tasks.push(tokio::spawn(async move {
                //    radio.handle_notification(notication).await
                //}));
                radio.handle_notification(notication).await?;
            }

            //for task in tasks {
            //    task.await?;
            //}
        }

        task::yield_now().await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let host = env::var("HOST")?;
    let pin = env::var("PIN")?;

    let radio = Radio::new(host, pin).await?;
    dbg!(radio);
    todo!();

    let radio = Arc::new(Radio::new(host, pin).await?);

    let task = tokio::spawn(keep_up_to_date(radio.clone()));

    let mut last_volume = *radio.audio.volume.volume.lock().await;
    for _ in 0..1_000 {
        let volume = *radio.audio.volume.volume.lock().await;
        if last_volume != volume {
            dbg!(volume);
            last_volume = volume;
        }
    }

    task.abort();

    //let concurrent_future = task::spawn(our_async_program(&radio));
    //concurrent_future.await?;
    Ok(())
}
