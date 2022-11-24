use anyhow::Result;
use opencv::videoio::VideoWriter;
use opencv::prelude::Mat;
use crate::cv::videoio::{VideoFileDirWriter, VideoFileWriter};
use crate::cv::VideoSelectedFileWriterTrait;


pub struct Writer {
    writer: VideoFileDirWriter<VideoFileWriter>,
}


impl Writer {
    pub fn new(writer: VideoFileDirWriter<VideoFileWriter>) -> Self {
        Self {
            writer
        }
    }

    pub fn save(&self, content: &Vec<Mat>) -> Result<()> {
        println!("WRITING");
        self.writer.save(content)
    }
}
