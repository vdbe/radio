use std::fmt::Display;
use tokio::sync::Mutex;

use fsapi::{FsApi, Node};

use crate::{Error, Radio};

#[derive(Debug)]
pub struct Power {
    pub(crate) state: Mutex<bool>,
}

impl Radio {
    pub async fn power_set(&self, power: bool) -> Result<(), Error> {
        self.power.set(power, &self.host, self.pin).await
    }
}

impl Power {
    pub async fn new<D: Display>(host: D, pin: u32) -> Result<Self, Error> {
        let state = match FsApi::get(Node::SysPower, &host, pin).await? {
            fsapi::Value::U8(state) => state == 1,
            _ => unreachable!("Power returns a U8"),
        };

        Ok(Self {
            state: Mutex::new(state),
        })
    }

    pub async fn set<D: Display>(&self, state: bool, host: D, pin: u32) -> Result<(), Error> {
        let lock = self.state.lock().await;
        let old_state = *lock;
        drop(lock);

        if state != old_state {
            FsApi::set(Node::SysPower, if old_state { 1 } else { 0 }, host, pin).await?;
        };

        *self.state.lock().await = state;

        Ok(())
    }

    pub async fn toggle<D: Display>(&self, host: D, pin: u32) -> Result<bool, Error> {
        let lock = self.state.lock().await;
        let new_state = !*lock;
        drop(lock);

        FsApi::set(Node::SysPower, if new_state { 1 } else { 0 }, host, pin).await?;

        *self.state.lock().await = new_state;

        Ok(new_state)
    }
}
