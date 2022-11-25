use anyhow::Result;
use std::mem;
use std::rc::Rc;
use super::super::handler::Handler;
use super::super::matdiff::MatDiff;
use super::writer::Writer;
use opencv::prelude::Mat;
use crate::camera::motion::state::{StatesConfig};
use super::state::State;
use super::state_watching::Watching;
use serde::Deserialize;


#[derive(Deserialize)]
#[serde(default)]
pub struct MotionDetect {
    diff: MatDiff,
    states_config: StatesConfig,
    #[serde(skip_deserializing)]
    prev_frame: Option<Mat>,
    #[serde(skip_deserializing)]
    state: Box<dyn State>,
}


impl MotionDetect {
    pub fn new(diff: MatDiff, states_config: StatesConfig) -> Self {
        Self {
            diff,
            states_config,
            prev_frame: None,
            state: Box::new(Watching::new())
        }
    }
}


impl Default for MotionDetect {
    fn default() -> Self {
        Self {
            diff: MatDiff::default(),
            states_config: StatesConfig::default(),
            prev_frame: None,
            state: Box::new(Watching::new())
        }
    }
}


impl Handler for MotionDetect  {
    fn new_frame(mut self, frame: &Mat) -> Result<Self> {
        let prev_frame: &Mat;

        match &self.prev_frame {
            Some(pf) => { prev_frame = pf; }
            None => {
                self.prev_frame = Some(frame.clone());
                prev_frame = &self.prev_frame.as_ref().unwrap()
            }
        }

        let diff = self.diff.diff(&prev_frame, &frame)?;
        let frames_differ = diff.are_different();

        let new_state = self.state.handle(frame, &self.states_config, frames_differ);

        match new_state {
            Ok(state) => { self.state = state }
            Err(E) => {
                self.state = Box::new(Watching::new());
                return Err(E)
            }
        }

        self.prev_frame = Some(frame.clone());

        Ok(self)
    }
}