use anyhow::Result;
use opencv::{
    prelude::*
};

pub trait Handler {
    fn new_frame(&mut self, frame: &Mat) -> Result<()>;
}