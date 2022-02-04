use std::fmt::Display;

use fsapi::{FsApi, Node};

use crate::Error;
use info::PlayerInfo;
mod info;

#[derive(Debug)]
pub struct Player {
    pub(crate) info: PlayerInfo,
    //pub(crate) status: Status,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Status {
    /// After switching between modes
    Loading = 0,

    Buffering = 1,

    Playing = 2,

    Paused = 3,

    /// When opening spotify
    Waiting = 5,

    /// Just guessing
    Disconnected = 6,

    Unknown = 10,
}

impl Player {
    pub async fn new<D: Display>(host: D, pin: D) -> Result<Self, Error> {
        let info = PlayerInfo::new(&host, &pin).await?;

        //let status = match FsApi::get(Node::PlayControl, &host, &pin).await? {
        //    fsapi::Value::U8(status) => Status::from(status),
        //    _ => unreachable!("SysPlayControl returns a U8"),
        //};

        Ok(Self { info })
    }

    async fn control_set<D: Display, O: Display>(option: O, host: D, pin: D) -> Result<(), Error> {
        FsApi::set(Node::PlayControl, option, &host, &pin).await?;

        Ok(())
    }

    pub async fn toggle<D: Display>(host: D, pin: D) -> Result<(), Error> {
        Self::control_set(0, host, pin).await
    }

    pub async fn next<D: Display>(host: D, pin: D) -> Result<(), Error> {
        Self::control_set(3, host, pin).await
    }

    pub async fn prev<D: Display>(host: D, pin: D) -> Result<(), Error> {
        Self::control_set(4, host, pin).await
    }
}

impl From<u8> for Status {
    fn from(status: u8) -> Self {
        use Status::*;

        match status {
            0 => Loading,
            1 => Buffering,
            2 => Playing,
            3 => Paused,
            5 => Waiting,
            6 => Disconnected,
            _ => Unknown,
        }
    }
}
