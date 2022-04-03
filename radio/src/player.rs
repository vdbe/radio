use std::fmt::Display;

use fsapi::{FsApi, Node};
use tokio::sync::Mutex;

use crate::Error;
use info::PlayerInfo;
use status::Status;

mod info;
mod status;

#[derive(Debug)]
pub struct Player {
    pub info: PlayerInfo,
    pub status: Mutex<Status>,
}

impl Player {
    pub async fn new<D: Display>(host: D, pin: u32) -> Result<Self, Error> {
        let info = PlayerInfo::new(&host, pin).await?;

        let status = Status::get(&host, pin).await?;

        Ok(Self {
            info,
            status: Mutex::new(status),
        })
    }

    async fn control_set<D: Display, O: Display>(
        option: O,
        host: D,
        pin: u32,
    ) -> Result<(), Error> {
        FsApi::set(Node::PlayControl, option, &host, pin).await?;

        Ok(())
    }

    pub async fn toggle<D: Display>(host: D, pin: u32) -> Result<(), Error> {
        Self::control_set(0, host, pin).await
    }

    pub async fn next<D: Display>(host: D, pin: u32) -> Result<(), Error> {
        Self::control_set(3, host, pin).await
    }

    pub async fn prev<D: Display>(host: D, pin: u32) -> Result<(), Error> {
        Self::control_set(4, host, pin).await
    }

    // TODO: I think there are more options than just: toggle, next, prev
}
