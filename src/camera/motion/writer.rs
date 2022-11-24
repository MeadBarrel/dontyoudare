use anyhow::Result;
use opencv::videoio::VideoWriter;
use opencv::prelude::Mat;
use log::*;
use crate::cv::videoio::{VideoFileDirWriter, VideoFileWriter};
use crate::cv::VideoSelectedFileWriterTrait;


pub struct Writer {
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
