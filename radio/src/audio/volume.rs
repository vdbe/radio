use std::fmt::Display;

use fsapi::{FsApi, Node};

use crate::{Error, Radio};

#[derive(Debug)]
pub struct Volume {
    pub(crate) max_volume: u32,
    pub(crate) volume: u32,
    pub(crate) muted: bool,
}

impl Radio {
    pub async fn volume_up(&mut self, change: i32) -> Result<(), Error> {
        self.audio.volume.up(change, &self.host, &self.pin).await
    }

    pub async fn volume_mute(&mut self, mute: bool) -> Result<(), Error> {
        self.audio.volume.mute(mute, &self.host, &self.pin).await
    }

    pub async fn volume_toggle(&mut self) -> Result<bool, Error> {
        self.audio.volume.toggle(&self.host, &self.pin).await
    }
}

impl Volume {
    pub async fn new<D: Display>(host: D, pin: D) -> Result<Self, Error> {
        let max_volume: u32 = match FsApi::get(Node::SysCapsVolumeSteps, &host, &pin).await? {
            fsapi::Value::U8(volume_steps) => {
                volume_steps.checked_sub(1).ok_or(Error::Empty)? as u32
            }
            _ => unreachable!("SysCapsVolumeSteps returns a U8"),
        };

        let volume: u32 = match FsApi::get(Node::SysAudioVolume, &host, &pin).await? {
            fsapi::Value::U8(volume) => volume as u32,
            _ => unreachable!("SysCapsVolume returns a U8"),
        };

        let muted = match FsApi::get(Node::SysAudioMute, &host, &pin).await? {
            fsapi::Value::U8(muted) => muted == 1,
            _ => unreachable!("SysCapsMute returns a U8"),
        };

        Ok(Self {
            max_volume,
            volume,
            muted,
        })
    }

    pub async fn set<D: Display>(&mut self, volume: u32, host: D, pin: D) -> Result<(), Error> {
        let volume = if volume > self.max_volume {
            self.max_volume
        } else {
            volume
        };

        FsApi::set(Node::SysAudioVolume, volume, host, pin).await?;

        self.volume = volume;

        Ok(())
    }

    pub async fn up<D: Display>(&mut self, change: i32, host: D, pin: D) -> Result<(), Error> {
        let new_volume = self.volume.checked_add_signed(change).unwrap_or(0);

        self.set(new_volume, host, pin).await
    }

    pub async fn mute<D: Display>(&mut self, mute: bool, host: D, pin: D) -> Result<(), Error> {
        if mute != self.muted {
            FsApi::set(Node::SysAudioMute, if mute { 1 } else { 0 }, host, pin).await?;
        };

        self.muted = mute;

        Ok(())
    }

    pub async fn toggle<D: Display>(&mut self, host: D, pin: D) -> Result<bool, Error> {
        self.mute(!self.muted, host, pin).await?;

        Ok(self.muted)
    }
}
