use std::rc::Rc;
use std::time::Duration;
use anyhow::Result;
use opencv::prelude::Mat;
use super::writer::Writer;


pub type StateResult = Result<Option<Box<dyn State>>>;

pub const STATE_UNCHANGED: StateResult = Ok(None);


pub struct StatesConfig {
    pub writer: Writer,
    pub min_video_duration: Duration,
    pub max_video_duration: Duration,
    pub max_idle_gap: Duration,
}


pub trait State {
    fn handle(&mut self, frame: &Mat, changed: bool) -> Result<Option<Box<dyn State>>> {
        if changed { self.handle_changed(frame) } else { self.handle_unchanged(frame) }
    }

    fn handle_changed(&mut self, frame: &Mat) -> Result<Option<Box<dyn State>>> {
        Ok(None)
    }

    fn handle_unchanged(&mut self, frame: &Mat) -> Result<Option<Box<dyn State>>> {
        Ok(None)
    }
}