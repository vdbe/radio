use std::fmt::Display;

use fsapi::{FsApi, Node};

use super::Player;
use crate::{Error, Radio};

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
    pub async fn player_toggle(&self) -> Result<(), Error> {
        // TODO: Check `self.mode` or `Node::PlayCaps` to check
        // if toggle is available for the current node.
        // Currently if not available you get an Error::InvalidValue back
        Player::toggle(&self.host, &self.pin).await
    }

    pub async fn player_next(&self) -> Result<(), Error> {
        // TODO: Check `self.mode` or `Node::PlayCaps` to check
        // if next is available for the current node.
        // Currently if not available you get an Error::InvalidValue back
        Player::next(&self.host, &self.pin).await
    }

    pub async fn player_prev(&self) -> Result<(), Error> {
        // TODO: Check `self.mode` or `Node::PlayCaps` to check
        // if next is available for the current node.
        // Currently if not available you get an Error::InvalidValue back
        Player::next(&self.host, &self.pin).await
    }

    pub async fn player_get_status(&self) -> Result<Status, Error> {
        Status::get(&self.host, &self.pin).await
    }
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
