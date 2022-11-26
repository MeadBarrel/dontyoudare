use std::time::Instant;
use opencv::prelude::Mat;
use log::*;
use super::state::*;
use super::state_recording_motion::RecordingMotion;
use super::state_watching::Watching;


pub struct RecordingIdle {
    since: Instant,
    frames: Vec<Mat>,
    collected_since: Instant,
    collected_frames_total: Vec<Mat>,
}


impl RecordingIdle {
    pub fn new(
        collected_since: Instant,
        collected_frames_total: Vec<Mat>) -> Self
    {
        debug!("Entering RecordingIdle state");
        Self {
            collected_since,
            collected_frames_total,
            since: Instant::now(),
            frames: Vec::default(),
        }
    }
}


impl State for RecordingIdle {
    fn handle_changed(mut self: Box<Self>, frame: &Mat, _: &StatesConfig) -> StateResult {
        self.frames.push(frame.clone());
        change_state(
            RecordingMotion::new(
                self.collected_since,
                [self.collected_frames_total, self.frames].concat()
            )
        )
    }

    fn handle_unchanged(mut self: Box<Self>, frame: &Mat, config: &StatesConfig) -> StateResult {
        let elapsed = self.since.elapsed();
        if elapsed < config.max_idle_gap {
            self.frames.push(frame.clone());
            return Ok(self)
        }
        info!("Total time elapsed: {:?}\nTotal motion captured: {:?}", self.collected_since.elapsed(), self.collected_since.elapsed() - elapsed);
        if self.collected_since.elapsed() - elapsed > config.min_video_duration {
            config.writer.save(&self.collected_frames_total)?;
        }
        change_state(Watching::new())
    }

}