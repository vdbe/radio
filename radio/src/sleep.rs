use std::fmt::Display;
use std::time::Duration;
use tokio::sync::Mutex;

use fsapi::{FsApi, Node};

use crate::{Error, Radio};

#[derive(Debug)]
pub struct Sleep {
    // TODO: replace with DateTime
    /// 0 is disabled
    pub(crate) sleep_in: Mutex<Duration>,
}

impl Radio {
    pub async fn sleep_in(&self, sleep_in: Duration) -> Result<(), Error> {
        FsApi::set(Node::SysSleep, sleep_in.as_secs(), &self.host, self.pin).await?;

        Ok(())
    }
}

impl Sleep {
    pub async fn new<D: Display>(host: D, pin: u32) -> Result<Self, Error> {
        let sleep_in = match FsApi::get(Node::SysSleep, &host, pin).await? {
            fsapi::Value::U32(duration) => Duration::from_secs(duration.into()),
            _ => unreachable!("Power returns a U32"),
        };

        Ok(Self {
            sleep_in: Mutex::new(sleep_in),
        })
    }

    pub async fn set<D: Display>(
        &mut self,
        sleep_in: Duration,
        host: D,
        pin: u32,
    ) -> Result<(), Error> {
        FsApi::set(Node::SysSleep, sleep_in.as_secs(), host, pin).await?;

        *self.sleep_in.lock().await = sleep_in;

        Ok(())
    }

    // TODO: a sleep_at(...) function
}
