use std::fmt::Display;

use clap::{Args, Parser, Subcommand};

use fsapi::{FsApi, Node, Value};

use crate::error::{Error, Result};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Turn radio on
    On,

    /// Turn radio off
    Off,

    /// Mute radio
    #[clap(visible_aliases = &["m"])]
    Mute,

    /// Unmute radio
    #[clap(name = "unmute", visible_aliases = &["M"])]
    UnMute,

    /// Change the Volume
    #[clap(subcommand, visible_aliases = &["v"])]
    Volume(Volume),

    #[clap(visible_aliases = &["f"])]
    Favorite(Number),
}

#[derive(Debug, Subcommand)]
pub enum Volume {
    /// Set the volume
    #[clap(visible_aliases = &["s", "="])]
    Set(Number),

    /// Increase volume (default 1)
    #[clap(visible_aliases = &["u", "+"])]
    Up(Number),

    /// Increase volume (default 1)
    #[clap(visible_aliases = &["d", "-"])]
    Down(Number),
}

#[derive(Debug, Args)]
pub struct Number {
    number: Option<u32>,
}

impl Default for Cli {
    fn default() -> Self {
        Self::parse()
    }
}

impl Command {
    pub async fn execute<D: Display + Copy>(&self, host: D, pin: u32) -> Result<()> {
        match self {
            Command::On => power(true, host, pin).await?,
            Command::Off => power(false, host, pin).await?,
            Command::Mute => mute(true, host, pin).await?,
            Command::UnMute => mute(false, host, pin).await?,
            Command::Volume(command) => match command {
                Volume::Set(Number { number }) => {
                    FsApi::set(
                        Node::SysAudioVolume,
                        number.ok_or(Error::InvalidCommand)?,
                        host,
                        pin,
                    )
                    .await?
                }
                Volume::Up(Number { number }) => {
                    FsApi::set(
                        Node::SysAudioVolume,
                        get_volume(host, pin)
                            .await?
                            .checked_add(number.unwrap_or(1))
                            .ok_or(Error::InvalidCommand)?,
                        host,
                        pin,
                    )
                    .await?
                }
                Volume::Down(Number { number }) => {
                    FsApi::set(
                        Node::SysAudioVolume,
                        get_volume(host, pin)
                            .await?
                            .checked_sub(number.unwrap_or(1))
                            .ok_or(Error::InvalidCommand)?,
                        host,
                        pin,
                    )
                    .await?
                }
            },
            Command::Favorite(Number { number }) => {
                FsApi::set(Node::NavState, 1, &host, pin).await?;
                FsApi::set(
                    Node::NavActionSelectPreset,
                    number.ok_or(Error::InvalidCommand)?,
                    &host,
                    pin,
                )
                .await?
            }
        }

        Ok(())
    }
}

async fn get_volume<D: Display>(host: D, pin: u32) -> Result<u32> {
    Ok(match FsApi::get(Node::SysAudioVolume, &host, pin).await? {
        Value::U8(volume) => volume as u32,
        _ => unreachable!("SysCapsVolume returns a U8"),
    })
}

async fn mute<D: Display>(mute: bool, host: D, pin: u32) -> Result<()> {
    Ok(FsApi::set(Node::SysAudioMute, if mute { 1 } else { 0 }, host, pin).await?)
}

async fn power<D: Display>(on: bool, host: D, pin: u32) -> Result<()> {
    Ok(FsApi::set(Node::SysPower, if on { 1 } else { 0 }, host, pin).await?)
}
