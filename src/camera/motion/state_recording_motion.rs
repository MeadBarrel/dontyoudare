use std::borrow::Borrow;
use std::mem;
use std::rc::Rc;
use std::time::Instant;
use opencv::prelude::Mat;
use log::*;
use super::state::*;
use super::state_watching::Watching;
use super::state_recording_idle::RecordingIdle;


pub struct RecordingMotion {
    since: Instant,
    frames: Vec<Mat>,
}


impl RecordingMotion {
    pub fn new(since: Instant, frames: Vec<Mat>) -> Self {
        debug!("(Re?)Entering RecordingMotion state; time elapsed: {:?}", since.elapsed());
        Self { since, frames }
    }
}


impl State for RecordingMotion {
    fn handle_changed(mut self: Box<Self>, frame: &Mat, config: &StatesConfig) -> StateResult {
        self.frames.push(frame.clone());

        if self.since.elapsed() > config.max_video_duration {
            config.writer.save(&self.frames)?;
            return change_state(Watching::new())
        }

        Ok(self)
    }
    fn handle_unchanged(self: Box<Self>, frame: &Mat, config: &StatesConfig) -> StateResult {
        change_state(
            RecordingIdle::new(self.since, self.frames)
        )
    }
}