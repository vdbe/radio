use std::fmt::Display;

use fsapi::{FsApi, Node};

use crate::{mode::Mode, Error, Radio};
use info::PlayerInfo;

mod info;

#[derive(Debug)]
pub struct Player {
    pub info: PlayerInfo,
    pub status: Status,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl Radio {
    pub async fn player_toggle(&mut self) -> Result<(), Error> {
        // TODO: Check `self.mode` or `Node::PlayCaps` to check
        // if toggle is available for the current node.
        // Currently if not available you get an Error::InvalidValue back
        Player::toggle(&self.host, &self.pin).await
    }

    pub async fn player_next(&mut self) -> Result<(), Error> {
        // TODO: Check `self.mode` or `Node::PlayCaps` to check
        // if next is available for the current node.
        // Currently if not available you get an Error::InvalidValue back
        Player::next(&self.host, &self.pin).await
    }

    pub async fn player_prev(&mut self) -> Result<(), Error> {
        // TODO: Check `self.mode` or `Node::PlayCaps` to check
        // if next is available for the current node.
        // Currently if not available you get an Error::InvalidValue back
        Player::next(&self.host, &self.pin).await
    }

    pub async fn player_get_status(&mut self) -> Result<Status, Error> {
        Status::get(&self.host, &self.pin).await
    }
}

impl Player {
    pub async fn new<D: Display>(host: D, pin: D) -> Result<Self, Error> {
        let info = PlayerInfo::new(&host, &pin).await?;

        let status = Status::get(&host, &pin).await?;

        Ok(Self { info, status })
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

    // TODO: I think there are more options than just: toggle, next, prev
}

impl Status {
    pub async fn get<D: Display>(host: D, pin: D) -> Result<Self, Error> {
        Ok(match FsApi::get(Node::PlayControl, host, pin).await? {
            fsapi::Value::U8(status) => Status::from(status),
            _ => unreachable!("SysPlayControl returns a U8"),
        })
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
