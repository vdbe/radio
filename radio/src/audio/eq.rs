use std::fmt::Display;

use crate::{Error, Radio};
use fsapi::{FsApi, Node, Value};

#[derive(Debug)]
pub struct Eq {
    pub(crate) preset: EqPreset,
    pub custom: EqCustom,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum EqPreset {
    Custom = 0,
    Normal = 1,
    Flat = 2,
    Jazz = 3,
    Rock = 4,
    Movie = 5,
    Classic = 6,
    Pop = 7,
    News = 8,
}

#[derive(Debug)]
pub struct EqCustom {
    pub(crate) loudness: bool,
    pub(crate) bass: i32,
    pub(crate) trebble: i32,
}

impl Radio {
    pub async fn eq_set(&mut self, preset: EqPreset) -> Result<(), Error> {
        self.audio.eq.set(preset, &self.host, &self.pin).await
    }

    pub async fn eq_custom_bass_set(&mut self, bass: i32) -> Result<(), Error> {
        self.audio
            .eq
            .custom
            .set_bass(bass, &self.host, &self.pin)
            .await
    }

    pub async fn eq_custom_trebble_set(&mut self, trebble: i32) -> Result<(), Error> {
        self.audio
            .eq
            .custom
            .set_bass(trebble, &self.host, &self.pin)
            .await
    }
}

impl Eq {
    pub async fn new<D: Display>(host: D, pin: D) -> Result<Self, Error> {
        let preset: EqPreset = match FsApi::get(Node::SysAudioEqPreset, &host, &pin).await? {
            Value::U8(preset) => preset.into(),
            _ => unreachable!("SysAudioEqloudness returns a U8"),
        };

        let custom = EqCustom::new(&host, &pin).await?;

        Ok(Self { preset, custom })
    }

    pub async fn set<D: Display>(
        &mut self,
        preset: EqPreset,
        host: D,
        pin: D,
    ) -> Result<(), Error> {
        FsApi::set(Node::SysAudioEqPreset, preset as u8, &host, &pin).await?;

        self.preset = preset;

        Ok(())
    }
}

impl EqCustom {
    pub async fn new<D: Display>(host: D, pin: D) -> Result<Self, Error> {
        let loudness = match FsApi::get(Node::SysAudioEqLoudness, &host, &pin).await? {
            Value::U8(state) => state == 1,
            _ => unreachable!("SysAudioEqloudness returns a U8"),
        };

        let bass = match FsApi::get(Node::SysAudioEqCustomParam0, &host, &pin).await? {
            Value::S16(bass) => bass.into(),
            _ => unreachable!("SysAudioEqCustomParam0 returns a S16"),
        };

        let trebble = match FsApi::get(Node::SysAudioEqCustomParam1, &host, &pin).await? {
            Value::S16(bass) => bass.into(),
            _ => unreachable!("SysAudioEqCustomParam1 returns a S16"),
        };

        Ok(Self {
            loudness,
            bass,
            trebble,
        })
    }

    pub async fn set_loudness<D: Display>(
        &mut self,
        loudness: bool,
        host: D,
        pin: D,
    ) -> Result<(), Error> {
        if loudness != self.loudness {
            FsApi::set(
                Node::SysAudioEqLoudness,
                if loudness { 1 } else { 0 },
                host,
                pin,
            )
            .await?;

            self.loudness = loudness;
        }

        Ok(())
    }

    pub async fn set_bass<D: Display>(&mut self, bass: i32, host: D, pin: D) -> Result<(), Error> {
        if bass != self.bass {
            if -7 <= bass && bass <= 7 {
                FsApi::set(Node::SysAudioEqCustomParam0, bass, host, pin).await?;
            } else {
                return Err(Error::InvalidValue);
            }

            self.bass = bass;
        }

        Ok(())
    }

    pub async fn set_trebble<D: Display>(
        &mut self,
        trebble: i32,
        host: D,
        pin: D,
    ) -> Result<(), Error> {
        if trebble != self.trebble {
            if -7 <= trebble && trebble <= 7 {
                FsApi::set(Node::SysAudioEqCustomParam1, trebble, host, pin).await?;
            } else {
                return Err(Error::InvalidValue);
            }

            self.trebble = trebble;
        }

        Ok(())
    }
}

impl From<u8> for EqPreset {
    fn from(preset: u8) -> Self {
        if preset <= Self::News as u8 {
            // UNSAFE: We checked above if the `preset` falls within
            // the EqPreset variants
            unsafe { ::std::mem::transmute(preset) }
        } else {
            // If the presets is not valid just return normal
            Self::Normal
        }
    }
}

impl Display for EqPreset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EqPreset::Custom => write!(f, "Custom"),
            EqPreset::Normal => write!(f, "Normal"),
            EqPreset::Flat => write!(f, "Flat"),
            EqPreset::Jazz => write!(f, "Jazz"),
            EqPreset::Rock => write!(f, "Rock"),
            EqPreset::Movie => write!(f, "Movie"),
            EqPreset::Classic => write!(f, "Classic"),
            EqPreset::Pop => write!(f, "Pop"),
            EqPreset::News => write!(f, "News"),
        }
    }
}
