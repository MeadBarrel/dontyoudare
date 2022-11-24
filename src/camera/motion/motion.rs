use std::rc::Rc;
use super::super::handler::Handler;
use super::super::matdiff::MatDiff;
use super::writer::Writer;
use opencv::prelude::Mat;
use crate::camera::motion::state::StatesConfig;
use super::state::State;
use super::state_watching::Watching;


pub struct MotionDetect {
    diff: MatDiff,
    prev_frame: Option<Mat>,
    state: Box<dyn State>,
}


impl MotionDetect {
    pub fn new(diff: MatDiff, config: Rc<StatesConfig>) -> Self {
        Self {
            diff,
            prev_frame: None,
            state: Box::new(Watching::new(config.clone()))
        }
    }

}

impl Handler for MotionDetect  {
    fn new_frame(&mut self, frame: &Mat) -> anyhow::Result<()> {
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

        match self.state.handle(frame, frames_differ)? {
            Some(state) => { self.state = state }
            None => {}
        };

        self.prev_frame = Some(frame.clone());

        Ok(())
    }
}