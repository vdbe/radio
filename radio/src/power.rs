use std::fmt::Display;

use fsapi::{FsApi, Node};

use crate::{Error, Radio};

#[derive(Debug)]
pub struct Power {
    pub(crate) state: bool,
}

impl Radio {
    pub async fn power_set(&mut self, power: bool) -> Result<(), Error> {
        self.power.set(power, &self.host, &self.pin).await
    }
}

impl Power {
    pub async fn new<D: Display>(host: D, pin: D) -> Result<Self, Error> {
        let state = match FsApi::get(Node::SysPower, &host, &pin).await? {
            fsapi::Value::U8(state) => state == 1,
            _ => unreachable!("Power returns a U8"),
        };

        Ok(Self { state })
    }

    pub async fn set<D: Display>(&mut self, state: bool, host: D, pin: D) -> Result<(), Error> {
        if state != self.state {
            FsApi::set(Node::SysPower, if state { 1 } else { 0 }, host, pin).await?;
        };

        self.state = state;

        Ok(())
    }

    pub async fn toggle<D: Display>(&mut self, host: D, pin: D) -> Result<bool, Error> {
        self.set(!self.state, host, pin).await?;

        Ok(self.state)
    }
}
