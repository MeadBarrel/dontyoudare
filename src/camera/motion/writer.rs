use anyhow::Result;
use opencv::prelude::Mat;
use log::*;
use crate::cv::videoio::VideoFileDirWriter;
use crate::cv::VideoSelectedFileWriterTrait;
use crate::signals::{Sender, Signal};


pub struct Writer {
    writer: VideoFileDirWriter,
    sender: Sender,
}


impl Writer {
    pub fn new(writer: VideoFileDirWriter, sender: Sender) -> Self {
        Self {
            writer,
            sender
        }
    }

    pub fn save(&self, content: &Vec<Mat>) -> Result<()> {
        debug!("Saving content of ({} frames)", content.len());
        let saved = self.writer.save(content)?;
        self.sender.send(Signal::MotionCaptured(saved))?;
        Ok(())
    }
}
