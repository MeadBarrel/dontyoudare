use anyhow::Result;
use opencv::{
    prelude::*
};

pub trait Handler {
    fn new_frame(self, frame: &Mat) -> Result<Self> where Self: Sized;
}
