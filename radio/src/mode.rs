use std::fmt::Display;

use fsapi::{FsApi, Node, Value};

use crate::{Error, Radio};

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum Mode {
    Internet = 0,
    Spotify = 1,

    /// Unused id (I think)
    /// so used as fallback
    NoIdea = 2,
    MusicPlayer = 3,
    Dab = 4,
    Fm = 5,
    AuxIn = 6,
}

impl Radio {
    pub async fn mode_set(&mut self, mode: Mode) -> Result<(), Error> {
        Mode::set(mode, &self.host, &self.pin).await?;

        self.mode = mode;
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

    pub async fn set<D: Display>(_mode: Mode, _host: D, _pin: D) -> Result<(), Error> {
        todo!("No idea how, guess I will figure it out at some point")
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
            Self::NoIdea
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Mode::*;
        match self {
            Internet => write!(f, "Internet"),
            Spotify => write!(f, "Spotify"),
            NoIdea => write!(f, "No idea"),
            MusicPlayer => write!(f, "Music Player"),
            Dab => write!(f, "DAB"),
            Fm => write!(f, "FM"),
            AuxIn => write!(f, "Aux in"),
        }
    }
}
