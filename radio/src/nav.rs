use std::fmt::Display;

use fsapi::{FsApi, Node, Value};
use tokio::sync::Mutex;

use crate::Error;

#[derive(Debug)]
pub struct Nav {
    pub state: Mutex<bool>,
    pub presets: Mutex<Vec<String>>,
}

impl Nav {
    pub async fn new<D: Display>(host: D, pin: D) -> Result<Self, Error> {
        FsApi::set(Node::NavState, 1, &host, &pin).await?;
        let state = true;

        let mut presets: Vec<String> = Vec::new();
        for preset in FsApi::get_item_list(Node::NavPresets, None, &host, &pin).await? {
            match &preset.fields[0].value {
                Value::Text(ref s) if s.is_empty() == false => presets.push(s.into()),
                Value::Text(_) => break,
                _ => panic!("NavPresets return a Vec<Field<Value::Text>>>"),
            };
        }

        Ok(Self {
            state: Mutex::new(state),
            presets: Mutex::new(presets),
        })
    }

    pub async fn preset_select<D: Display>(preset: u32, host: D, pin: D) -> Result<(), Error> {
        FsApi::set(Node::NavState, preset, &host, &pin).await?;

        Ok(())
    }
}
