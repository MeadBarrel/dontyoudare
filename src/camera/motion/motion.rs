use anyhow::Result;
use std::mem;
use std::rc::Rc;
use super::super::handler::Handler;
use super::super::matdiff::MatDiff;
use super::writer::Writer;
use opencv::prelude::Mat;
use crate::camera::motion::state::{StatesConfig};
use crate::signals::*;
use super::state::State;
use super::state_watching::Watching;
use serde::Deserialize;
use crate::camera::state::StatesConfigConfig;
use crate::cv::VideoFileDirWriter;


#[derive(Default, Deserialize)]
#[serde(default)]
pub struct MotionDetectConfig {
    diff: MatDiff,
    states_config: StatesConfigConfig,
    writer: VideoFileDirWriter,
}


impl MotionDetectConfig {
    pub fn create(self, sender: Sender) -> MotionDetect {
        MotionDetect::new(
            self.diff,
            StatesConfig {
                writer: Writer::new(self.writer, sender),
                min_video_duration: self.states_config.min_video_duration,
                max_video_duration: self.states_config.max_video_duration,
                max_idle_gap: self.states_config.max_idle_gap,
            }
        )
    }
}


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
            Err(E) => {
                self.state = Box::new(Watching::new());
                return Err(E)
            }
        }

        self.prev_frame = Some(frame.clone());

        Ok(self)
    }
}