#![feature(mixed_integer_ops)]
use fsapi::{FsApi, Node, Notification, SessionID, Value};
use tokio::sync::Mutex;

use audio::eq::EqPreset;
use audio::Audio;
pub use error::Error;
use mode::Mode;
use nav::Nav;
use player::Player;
use power::Power;
use sleep::Sleep;

pub mod audio;
pub mod error;
pub mod mode;
pub mod nav;
pub mod player;
pub mod power;
pub mod sleep;

#[derive(Debug)]
pub struct Radio {
    pub(crate) host: String,
    pub(crate) pin: String,
    pub(crate) session_id: SessionID,
    pub audio: Audio,
    pub player: Player,
    pub nav: Nav,
    pub power: Power,
    pub sleep: Sleep,
    pub mode: Mutex<Mode>,
}

impl Radio {
    pub async fn new<T: ToString>(host: T, pin: T) -> Result<Self, Box<dyn std::error::Error>> {
        let host = host.to_string();
        let pin = pin.to_string();

        let session_id = FsApi::create_session(&host, &pin).await?;

        let audio = Audio::new(&host, &pin).await?;

        let player = Player::new(&host, &pin).await?;

        let nav = Nav::new(&host, &pin).await?;

        let power = Power::new(&host, &pin).await?;

        let sleep = Sleep::new(&host, &pin).await?;

        let mode = Mode::new(&host, &pin).await?;

        Ok(Self {
            host,
            pin,
            session_id,
            audio,
            player,
            nav,
            power,
            sleep,
            mode: Mutex::new(mode),
        })
    }

    pub async fn get_notifications(&self) -> Result<Option<Vec<Notification>>, Error> {
        //let host = radio.host.clone();
        //let pin = radio.pin.clone();
        //let session_id = radio.session_id;

        let notifications =
            FsApi::get_notifications(self.session_id, &self.host, &self.pin).await?;

        Ok(notifications)
    }

    pub async fn handle_notification(&self, notification: Notification) -> Result<(), Error> {
        use Node::*;

        match notification.node {
            SysAudioVolume => {
                if let Value::U8(volume) = notification.value {
                    *self.audio.volume.volume.lock().await = volume as u32;
                }
            }
            SysAudioMute => {
                if let Value::U8(mute) = notification.value {
                    *self.audio.volume.muted.lock().await = mute == 1;
                }
            }
            SysPower => {
                if let Value::U8(state) = notification.value {
                    *self.power.state.lock().await = state == 1;
                }
            }
            SysAudioEqPreset => {
                if let Value::U8(preset) = notification.value {
                    *self.audio.eq.preset.lock().await = EqPreset::from(preset);
                }
            }
            SysAudioEqLoudness => {
                if let Value::U8(loudness) = notification.value {
                    *self.audio.eq.custom.loudness.lock().await = loudness == 1;
                }
            }
            SysAudioEqCustomParam0 => {
                if let Value::S16(bass) = notification.value {
                    *self.audio.eq.custom.bass.lock().await = bass.into();
                }
            }
            SysAudioEqCustomParam1 => {
                if let Value::S16(trebble) = notification.value {
                    *self.audio.eq.custom.trebble.lock().await = trebble.into();
                }
            }
            PlayInfoName => {
                if let Value::Text(name) = notification.value {
                    *self.player.info.name.lock().await = name;
                }
            }
            PlayInfoText => {
                if let Value::Text(text) = notification.value {
                    *self.player.info.text.lock().await = text;
                }
            }
            PlayInfoAlbum => {
                if let Value::Text(album) = notification.value {
                    *self.player.info.album.lock().await = album
                }
            }
            PlayInfoArtist => {
                if let Value::Text(artist) = notification.value {
                    *self.player.info.artist.lock().await = artist;
                }
            }
            PlayInfoDuration => {
                if let Value::U32(duration) = notification.value {
                    *self.player.info.duration.lock().await =
                        std::time::Duration::from_millis(duration.into());
                }
            }
            PlayInfoGraphicUri => {
                if let Value::Text(graphic_uri) = notification.value {
                    *self.player.info.graphic_uri.lock().await = graphic_uri;
                }
            }
            SysMode => {
                if let Value::U32(mode) = notification.value {
                    *self.mode.lock().await = mode.into();
                }
            }
            PlayStatus => {
                if let Value::U8(status) = notification.value {
                    *self.player.status.lock().await = status.into();
                }
            }
            PlayServiceIdsEcc => (),
            SysState => (),
            SysClockLocalTime => (),
            SysClockLocalDate => (),

            node => panic!("Update node: {:?}", node),
        }

        Ok(())
    }
}
