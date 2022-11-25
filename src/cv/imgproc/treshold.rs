use opencv::core::{ToInputArray, ToOutputArray};
use opencv::Error;
use opencv::imgproc::threshold;
use serde::Deserialize;
use super::traits::*;


pub struct Threshold {
    thresh: f64,
    maxval: f64,
    typ: i32
}


impl Threshold {
    pub fn new(thresh: f64, maxval: f64, typ: i32) -> Self {
        Self {
            thresh, maxval, typ
        }
    }
}


impl OneToOneConvert for Threshold {
    fn dest(&self, src: &dyn ToInputArray, dest: &mut dyn ToOutputArray) -> opencv::Result<()> {
        threshold(
            src,
            dest,
            self.thresh,
            self.maxval,
            self.typ
        )?;
        Ok(())
    }
}


impl OneToOneConvertPrep for Threshold {}
