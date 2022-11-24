use std::rc::Rc;
use std::time::Instant;
use opencv::prelude::{anyTraitConst, Mat};
use log::*;
use super::state::*;
use super::state_recording_motion::RecordingMotion;


pub struct Watching {
    config: Rc<StatesConfig>,
}


impl Watching {
    pub fn new(config: Rc<StatesConfig>) -> Self {
        debug!("Entering Watching state");
        Self { config }
    }
}


impl State for Watching {
    fn handle_changed(&mut self, frame: &Mat) -> anyhow::Result<Option<Box<dyn State>>> {
        Ok(
            Some(
                Box::new(
                    RecordingMotion::new(
                        self.config.clone(),
                        Instant::now(),
                        vec![frame.clone()]
                    )
                )
            )
        )
    }
}
