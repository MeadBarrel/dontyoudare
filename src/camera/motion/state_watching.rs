use std::time::Instant;
use opencv::prelude::Mat;
use log::*;
use super::state::*;
use super::state_recording_motion::RecordingMotion;


pub struct Watching;


impl Watching {
    pub fn new() -> Self {
        debug!("Entering Watching state");
        Watching {}
    }
}


impl State for Watching {
    fn handle_changed(self: Box<Self>, frame: &Mat, _config: &StatesConfig) -> StateResult {
        change_state(RecordingMotion::new(Instant::now(), vec![frame.clone()]))
    }
    fn handle_unchanged(self: Box<Self>, _: &Mat, _config: &StatesConfig) -> StateResult {
        Ok(self)
    }

}
