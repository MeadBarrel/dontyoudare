use anyhow::Result;
use super::super::handler::Handler;
use super::super::matdiff::MatDiff;
use super::writer::Writer;
use opencv::prelude::Mat;
use crate::camera::motion::state::{StatesConfig};
use crate::signals::*;
use super::state::State;
use super::state_watching::Watching;
use crate::cv::VideoFileDirWriter;


pub struct MotionDetect {
    diff: MatDiff,
    states_config: StatesConfig,
    prev_frame: Option<Mat>,
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

        let frames_differ = self.diff.diff(&prev_frame, &frame)?;

        let new_state = self.state.handle(frame, &self.states_config, frames_differ);

        match new_state {
            Ok(state) => { self.state = state }
            Err(e) => {
                self.state = Box::new(Watching::new());
                return Err(e)
            }
        }

        self.prev_frame = Some(frame.clone());

        Ok(self)
    }
}