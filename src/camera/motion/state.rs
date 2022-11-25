use std::rc::Rc;
use std::time::Duration;
use anyhow::Result;
use opencv::prelude::Mat;
use serde::Deserialize;
use crate::camera::MatDiffPipe;
use super::writer::Writer;


pub type StateResult = Result<Box<dyn State>>;

pub fn change_state(state: impl State + 'static) -> StateResult {
    Ok(Box::new(state))
}


#[derive(Deserialize)]
#[serde(default)]
pub struct StatesConfig {
    pub writer: Writer,
    pub min_video_duration: Duration,
    pub max_video_duration: Duration,
    pub max_idle_gap: Duration,
}


impl Default for StatesConfig {
    fn default() -> Self {
        Self {
            writer: Writer::default(),
            min_video_duration: Duration::from_secs(1),
            max_video_duration: Duration::from_secs(15),
            max_idle_gap: Duration::from_secs(1),
        }
    }
}


pub trait State {
    fn handle(self: Box<Self>, frame: &Mat, config: &StatesConfig, changed: bool) -> StateResult {
        match changed {
            true => self.handle_changed(frame, config),
            false => self.handle_unchanged(frame, config)
        }
    }

    fn handle_changed(self: Box<Self>, frame: &Mat, config: &StatesConfig) -> StateResult;
    fn handle_unchanged(self: Box<Self>, frame: &Mat, config: &StatesConfig) -> StateResult;
}

