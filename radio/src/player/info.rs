use std::fmt::Display;
use std::time::Duration;

use fsapi::{FsApi, Node, Value};

use crate::Error;

#[derive(Debug)]
pub struct PlayerInfo {
    /// First line of display
    pub(crate) name: String,

    /// Second line of display
    pub(crate) text: String,

    pub(crate) album: String,
    pub(crate) artist: String,

    pub(crate) duration: Duration,

    // TODO: Figure out when to reset since
    // not all stations send it
    pub(crate) graphic_uri: String,
}

impl PlayerInfo {
    pub async fn new<D: Display>(host: D, pin: D) -> Result<Self, Error> {
        let name = match FsApi::get(Node::PlayInfoName, &host, &pin).await? {
            Value::Text(name) => name,
            _ => unreachable!("PlayInfoName returns a Text"),
        };

        let text = match FsApi::get(Node::PlayInfoText, &host, &pin).await? {
            Value::Text(text) => text,
            _ => unreachable!("PlayInfoText returns a Text"),
        };

        let album = match FsApi::get(Node::PlayInfoAlbum, &host, &pin).await? {
            Value::Text(album) => album,
            _ => unreachable!("PlayInfoAlbum returns a Text"),
        };

        let artist = match FsApi::get(Node::PlayInfoArtist, &host, &pin).await? {
            Value::Text(artist) => artist,
            _ => unreachable!("PlayInfoArtist returns a Text"),
        };

        let duration = match FsApi::get(Node::PlayInfoDuration, &host, &pin).await? {
            Value::U32(duration) => Duration::from_millis(duration.into()),
            _ => unreachable!("PlayInfoDuration returns a U32"),
        };

        let graphic_uri = match FsApi::get(Node::PlayInfoGraphicUri, &host, &pin).await? {
            Value::Text(graphic_uri) => graphic_uri,
            _ => unreachable!("PlayInfoGraphicUri returns a Text"),
        };

        Ok(Self {
            name,
            text,
            album,
            artist,
            duration,
            graphic_uri,
        })
    }
}