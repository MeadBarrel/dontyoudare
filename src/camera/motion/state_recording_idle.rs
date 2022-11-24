use std::mem;
use std::rc::Rc;
use std::time::Instant;
use anyhow::Result;
use opencv::prelude::Mat;
use log::*;
use super::state::*;
use super::state_recording_motion::RecordingMotion;
use super::state_watching::Watching;


pub struct RecordingIdle {
    config: Rc<StatesConfig>,
    since: Instant,
    frames: Vec<Mat>,
    collected_since: Instant,
    collected_frames_total: Vec<Mat>,
}


impl RecordingIdle {
    pub fn new(
        config: Rc<StatesConfig>,
        collected_since: Instant,
        collected_frames_total: Vec<Mat>) -> Self
    {
        debug!("Entering RecordingIdle state");
        Self {
            config,
            collected_since,
            collected_frames_total,
            since: Instant::now(),
            frames: Vec::default(),
        }
    }
}


impl State for RecordingIdle {
    fn handle_changed(&mut self, frame: &Mat) -> Result<Option<Box<dyn State>>> {
        let mut new_vec = mem::take(&mut self.collected_frames_total);
        new_vec.extend(mem::take(&mut self.frames));
        new_vec.push(frame.clone());

        Ok(Some(Box::new(
            RecordingMotion::new(
                self.config.clone(),
                self.collected_since.clone(),
                new_vec,
            )
        )))
    }

    fn handle_unchanged(&mut self, frame: &Mat) -> Result<Option<Box<dyn State>>>
    {
        self.frames.push(frame.clone());
        let cannot_wait = self.since.elapsed() > self.config.max_idle_gap;
        let video_too_short = self.collected_since.elapsed() - self.since.elapsed()
            < self.config.min_video_duration;

        if cannot_wait {
            if !video_too_short {
                self.config.writer.save(&self.collected_frames_total);
            }
            return Ok(Some(Box::new(Watching::new(self.config.clone()))));
        }

        STATE_UNCHANGED
    }
}