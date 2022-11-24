use std::borrow::Borrow;
use std::mem;
use std::rc::Rc;
use std::time::Instant;
use opencv::prelude::Mat;
use super::state::*;
use super::state_watching::Watching;
use super::state_recording_idle::RecordingIdle;


pub struct RecordingMotion {
    config: Rc<StatesConfig>,
    since: Instant,
    frames: Vec<Mat>,
}


impl RecordingMotion {
    pub fn new(config: Rc<StatesConfig>, since: Instant, frames: Vec<Mat>) -> Self {
        println!("Recording");
        Self { config, since, frames }
    }
}


impl State for RecordingMotion {
    fn handle_changed(&mut self, frame: &Mat) -> anyhow::Result<Option<Box<dyn State>>> {
        let config = self.config.as_ref();
        self.frames.push(frame.clone());

        if self.since.elapsed() > config.max_video_duration {
            config.writer.save(&self.frames);
            return Ok(Some(Box::new(Watching::new(self.config.clone()))));
        }

        Ok(None)
    }

    fn handle_unchanged(&mut self, frame: &Mat) -> anyhow::Result<Option<Box<dyn State>>> {
        Ok(
            Some(
                Box::new(
                    RecordingIdle::new(
                        self.config.clone(),
                        self.since.clone(),
                        mem::take(&mut self.frames),
                    )
                )
            )
        )
    }
}