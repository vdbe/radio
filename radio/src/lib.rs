#![feature(mixed_integer_ops)]

use fsapi::{FsApi, Node, SessionID, Value};

use audio::eq::EqPreset;
use audio::Audio;
use error::Error;
use mode::Mode;
use player::Player;
use power::Power;
use sleep::Sleep;

pub mod audio;
pub mod error;
pub mod mode;
pub mod player;
pub mod power;
pub mod sleep;

#[derive(Debug)]
pub struct Radio {
    host: String,
    pin: String,
    session_id: SessionID,
    audio: Audio,
    player: Player,
    power: Power,
    sleep: Sleep,
    mode: Mode,
}

impl Radio {
    pub async fn new<T: ToString>(host: T, pin: T) -> Result<Self, Box<dyn std::error::Error>> {
        let host = host.to_string();
        let pin = pin.to_string();

        let session_id = FsApi::create_session(&host, &pin).await?;

        let audio = Audio::new(&host, &pin).await?;

        let player = Player::new(&host, &pin).await?;

        let power = Power::new(&host, &pin).await?;

        let sleep = Sleep::new(&host, &pin).await?;

        let mode = Mode::new(&host, &pin).await?;

        Ok(Self {
            host,
            pin,
            session_id,
            audio,
            player,
            power,
            sleep,
            mode,
        })
    }

    pub async fn update_state(&mut self) -> Result<(), Error> {
        use Node::*;
        let notifications =
            FsApi::get_notifications(self.session_id, &self.host, &self.pin).await?;

        for notifcation in notifications {
            match dbg!(notifcation.node) {
                SysAudioVolume => {
                    if let Value::U8(volume) = notifcation.value {
                        self.audio.volume.volume = volume as u32;
                    }
                }
                SysAudioMute => {
                    if let Value::U8(mute) = notifcation.value {
                        self.audio.volume.muted = mute == 1;
                    }
                }
                SysPower => {
                    if let Value::U8(state) = notifcation.value {
                        self.power.state = state == 1;
                    }
                }
                SysAudioEqPreset => {
                    if let Value::U8(preset) = notifcation.value {
                        self.audio.eq.preset = dbg!(EqPreset::from(preset));
                    }
                }
                SysAudioEqLoudness => {
                    if let Value::U8(loudness) = notifcation.value {
                        self.audio.eq.custom.loudness = loudness == 1;
                    }
                }
                SysAudioEqCustomParam0 => {
                    if let Value::S16(bass) = notifcation.value {
                        self.audio.eq.custom.bass = bass.into();
                    }
                }
                SysAudioEqCustomParam1 => {
                    if let Value::S16(trebble) = notifcation.value {
                        self.audio.eq.custom.trebble = trebble.into();
                    }
                }
                PlayInfoName => {
                    if let Value::Text(name) = notifcation.value {
                        self.player.info.name = name;
                    }
                }
                PlayInfoText => {
                    if let Value::Text(text) = notifcation.value {
                        self.player.info.text = text;
                    }
                }
                PlayInfoAlbum => {
                    if let Value::Text(album) = notifcation.value {
                        self.player.info.album = album;
                    }
                }
                PlayInfoArtist => {
                    if let Value::Text(artist) = notifcation.value {
                        self.player.info.artist = artist;
                    }
                }
                PlayInfoDuration => {
                    if let Value::U32(duration) = notifcation.value {
                        self.player.info.duration =
                            std::time::Duration::from_millis(duration.into());
                    }
                }
                PlayInfoGraphicUri => {
                    if let Value::Text(graphic_uri) = notifcation.value {
                        self.player.info.graphic_uri = graphic_uri;
                    }
                }
                SysMode => {
                    if let Value::U32(mode) = notifcation.value {
                        self.mode = mode.into();
                    }
                }
                PlayServiceIdsEcc => (),
                PlayStatus => (),
                SysState => (),

                node => panic!("Update node: {:?}", node),
            }
        }

        Ok(())
    }
}
