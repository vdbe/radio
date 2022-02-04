use std::fmt::Display;

use crate::Error;
use eq::Eq;
use volume::Volume;

pub mod eq;
pub mod volume;

#[derive(Debug)]
pub struct Audio {
    pub volume: Volume,
    pub eq: Eq,
}

impl Audio {
    pub async fn new<D: Display>(host: D, pin: D) -> Result<Self, Error> {
        let volume = Volume::new(&host, &pin).await?;

        let eq = Eq::new(&host, &pin).await?;

        Ok(Self { volume, eq })
    }
}
