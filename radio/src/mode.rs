use std::fmt::Display;

use fsapi::{FsApi, Node, Value};

use crate::{Error, Radio};

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum Mode {
    Internet = 0,
    Spotify = 1,
    Dmr = 2,
    MusicPlayer = 3,
    Dab = 4,
    Fm = 5,
    AuxIn = 6,

    /// Fallback do not use this
    FallBack = 7,
}

impl Radio {
    pub async fn mode_set(&self, mode: Mode) -> Result<(), Error> {
        Mode::set(mode, &self.host, &self.pin).await?;

        //self.mode = mode;
        Ok(())
    }
}

impl Mode {
    pub async fn new<D: Display>(host: D, pin: D) -> Result<Self, Error> {
        let mode = match FsApi::get(Node::SysMode, &host, &pin).await? {
            Value::U32(mode) => mode.into(),
            _ => unreachable!("SysMode returns a U32"),
        };

        Ok(mode)
    }

    pub async fn set<D: Display>(mode: Mode, host: D, pin: D) -> Result<(), Error> {
        if mode == Mode::FallBack {
            return Err(Error::InvalidValue);
        }

        dbg!(FsApi::set(Node::SysMode, mode as u32, host, pin).await?);

        Ok(())
    }
}

impl From<u32> for Mode {
    fn from(mode: u32) -> Self {
        if mode <= Self::AuxIn as u32 {
            // UNSAFE: We checked above if the `mode` falls within
            // the Mode variants
            unsafe { ::std::mem::transmute(mode) }
        } else {
            // If the presets is not valid just return normal
            Self::FallBack
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Mode::*;
        match self {
            Internet => write!(f, "Internet"),
            Spotify => write!(f, "Spotify"),
            Dmr => write!(f, "DMR"),
            MusicPlayer => write!(f, "Music Player"),
            Dab => write!(f, "DAB"),
            Fm => write!(f, "FM"),
            AuxIn => write!(f, "Aux in"),
            FallBack => write!(f, "Fallback"),
        }
    }
}
