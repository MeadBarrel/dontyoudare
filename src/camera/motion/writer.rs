use std::ptr::write;
use anyhow::Result;
use opencv::videoio::VideoWriter;
use opencv::prelude::Mat;
use log::*;
use serde::Deserialize;
use crate::cv::videoio::{VideoFileDirWriter, VideoFileWriter};
use crate::cv::VideoSelectedFileWriterTrait;


#[derive(Default, Deserialize)]
pub struct Writer {
    #[serde(flatten)]
    writer: VideoFileDirWriter
}


impl Writer {
    pub fn new(writer: VideoFileDirWriter) -> Self {
        Self {
            writer
        }
    }

    pub fn save(&self, content: &Vec<Mat>) -> Result<()> {
        debug!("Saving content of ({} frames)", content.len());
        self.writer.save(content)
    }
}
