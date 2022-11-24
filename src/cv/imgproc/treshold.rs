use opencv::core::{ToInputArray, ToOutputArray};
use opencv::Error;
use opencv::imgproc::threshold;
use super::traits::*;


#[derive(Clone, Copy)]
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

    pub fn with_thresh(&self, thresh: f64) -> Self {
        Self {
            thresh,
            ..*self
        }
    }

    pub fn with_maxval(&self, maxval: f64) -> Self {
        Self {
            maxval,
            ..*self
        }
    }

    pub fn with_typ(&self, typ: i32) -> Self {
        Self {
            typ,
            ..*self
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
