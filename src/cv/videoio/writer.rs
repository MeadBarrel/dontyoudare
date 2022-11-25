use std::fs::create_dir_all;
use std::path::Path;
use std::time::Duration;
use chrono::prelude::*;
use anyhow::{Error, Result};
use opencv::{
    prelude::*,
    videoio,
    core::ToInputArray,
};
use opencv;
use opencv::core::{Size, Size_};
use opencv::imgproc::{InterpolationFlags, resize};
use opencv::videoio::VideoWriter;
use log::*;
use serde::Deserialize;
use crate::config::deserialize_fourcc;


pub trait VideoFileWriterTrait {
    //! Writes a video file to `path`
    fn save(&self, path: &str, content: &Vec<Mat>) -> Result<()>;
}


pub trait VideoSelectedFileWriterTrait {
    //! Writes a video file
    fn save(&self, content: &Vec<Mat>) -> Result<()>;
}


/// Defines the FPS for VideoFileWriter
pub enum FPSConfig {
    // Static FPS
    Static(f64),
    // Try to derive FPS as (number of frames) / Duration
    Derived(Duration),
}


/// Defines a frame size for VideoFileWriter
pub enum FrameSizeConfig {
    // Static frame size - don't try to resize non-matching frame
    Static(Size),

    // Try to resize non-matching frames to the defined frame size
    //
    // First parameter is the desired frame size, second is interpolation method
    // See: https://docs.rs/opencv/latest/opencv/imgproc/enum.InterpolationFlags.html
    Resize(Size, i32),

    // The frame size will match the size of the first frame
    Derive,

    // Combines Derive and Resize - all frames will be resized to the size of the first frame
    DeriveResize(i32),
}


/// Video file writer class
///
/// Uses opencv::videoio::VideoWriter to write a vector of frames to a file
///
/// # Parameters
///
///     - filename: The name of the file to write to
///     - fourcc: For example, VideoWriter::fourcc(‘P’,‘I’,‘M’,‘1’) is a MPEG-1 codec,
///               VideoWriter::fourcc(‘M’,‘J’,‘P’,‘G’) is a motion-jpeg codec etc.
///               List of codes can be obtained at Video Codecs by FOURCC page.
///
///     - fps: Framerate of the created video. See #FPSConfig
///     - frame_size: Frame size of the created video. See #FrameSizeConfig
///     - api_preference: The apiPreference parameter allows to specify API backends to use.
///                       Can be used to enforce a specific reader implementation if multiple are
///                       available: e.g. cv::CAP_FFMPEG or cv::CAP_GSTREAMER.
///     - is_color: If it is not zero, the encoder will expect and encode color frames,
///                 otherwise it will work with grayscale frames.
///
#[derive(Deserialize)]
#[serde(default)]
pub struct VideoFileWriter {
    #[serde(deserialize_with="deserialize_fourcc")]
    fourcc: i32,
    #[serde(skip_deserializing)]
    fps: FPSConfig,
    #[serde(skip_deserializing)]
    frame_size: FrameSizeConfig,
    api_preference: Option<i32>,
    is_color: bool,
}


#[derive(Deserialize)]
#[serde(default)]
pub struct VideoFileDirWriter {
    writer: VideoFileWriter,
    filename_format: String,
    folder: String,
}


impl Default for VideoFileDirWriter {
    fn default() -> Self {
        Self {
            writer: VideoFileWriter::default(),
            filename_format: "%Y-%m-%d-%H-%M-%S.avi".to_owned(),
            folder: "output".to_owned(),
        }
    }
}


impl VideoFileWriter {
    pub fn new(
        fourcc: i32,
        fps: FPSConfig,
        frame_size: FrameSizeConfig,
        api_preference: Option<i32>,
        is_color: bool
    ) -> Self {
        Self {
            fourcc, fps, frame_size, api_preference, is_color
        }
    }

    fn resize_frame(&self, frame: &Mat, size: Size, interpolation: i32) -> Result<Mat> {
        let mut resized_frame = Mat::default();
        resize(
            &frame,
            &mut resized_frame,
            size,
            0_f64,
            0_f64,
            interpolation
        )?;
        Ok(resized_frame)
    }

    fn create_writer(&self, filename: &str, fps: f64, frame_size: Size, is_color: bool) -> opencv::Result<VideoWriter> {
        match self.api_preference {
            Some(api_preference) => {
                VideoWriter::new_with_backend(
                    filename,
                    api_preference,
                    self.fourcc,
                    fps,
                    frame_size,
                    is_color
                )
            }
            None => {
                VideoWriter::new(
                    filename,
                    self.fourcc,
                    fps,
                    frame_size,
                    is_color
                )
            }
        }
    }
}


impl VideoFileWriterTrait for VideoFileWriter {
    fn save(&self, path: &str, content: &Vec<Mat>) -> Result<()> {
        if content.is_empty() {
            return Err(Error::msg("Cannot write video file: no content"))
        }

        let fps = match self.fps {
            FPSConfig::Static(fps) => { fps }
            FPSConfig::Derived(duration) => { content.len() as f64 / duration.as_secs() as f64}
        };

        let frame_size = match self.frame_size {
            FrameSizeConfig::Static(size) | FrameSizeConfig::Resize(size, _) => { size }
            FrameSizeConfig::DeriveResize(_) | FrameSizeConfig::Derive => {
                content.first().unwrap().size()?
            }
        };
        let resize_interpolation = match self.frame_size {
            FrameSizeConfig::Static(_) | FrameSizeConfig::Derive => { None }
            FrameSizeConfig::Resize(_, i) | FrameSizeConfig::DeriveResize(i) => { Some(i) }
        };

        let mut writer = self.create_writer(path, fps, frame_size, self.is_color)?;


        for frame in content {
            match resize_interpolation {
                Some(i) => {
                    let new_frame = self.resize_frame(frame, frame_size, i)?;
                    writer.write(&new_frame)?;
                }
                None => { writer.write(frame)?; }
            }
        };

        writer.release();

        Ok(())
    }
}


impl Default for VideoFileWriter {
    fn default() -> Self {
        Self {
            fourcc: VideoWriter::fourcc('D', 'I', 'V', 'X').unwrap(),
            fps: FPSConfig::Static(24.),
            frame_size: FrameSizeConfig::DeriveResize(InterpolationFlags::INTER_LANCZOS4 as i32),
            api_preference: None,
            is_color: true
        }
    }
}


impl VideoFileDirWriter {
    pub fn new(writer: VideoFileWriter, filename_format: &str, folder: &str) -> Self {
        Self {
            writer,
            filename_format: filename_format.to_string(),
            folder: folder.to_string()
        }
    }
}


impl VideoSelectedFileWriterTrait for VideoFileDirWriter {
    fn save(&self, content: &Vec<Mat>) -> Result<()> {
        let folder_path = Path::new(&self.folder);
        create_dir_all(folder_path)?;

        let filename = Utc::now().format(&self.filename_format).to_string();

        let fnp = Path::new(&filename);
        let joined = folder_path.join(fnp);
        let joined_str = joined.to_str().ok_or(Error::msg("Improper filename"))?;

        debug!("Saving {} frames to: {}", content.len(), &joined_str);

        Ok(self.writer.save(&joined_str, content)?)
    }
}


pub mod config {
    use std::time::Duration;
    use opencv::core::Size;
    use opencv::imgproc::InterpolationFlags;
    use opencv::videoio::VideoWriter;
    use serde::Deserialize;
    use crate::cv::VideoFileWriter;
    use super::{FPSConfig, FrameSizeConfig};

    #[derive(Deserialize)]
    pub enum InterpolationConfig {
        Lanczos4 = InterpolationFlags::INTER_LANCZOS4 as isize,
        Cubic = InterpolationFlags::INTER_CUBIC as isize,
        Area = InterpolationFlags::INTER_AREA as isize,
        Linear = InterpolationFlags::INTER_LINEAR as isize,
        LinearExact = InterpolationFlags::INTER_LINEAR_EXACT as isize,
        Max = InterpolationFlags::INTER_MAX as isize,
        Nearest = InterpolationFlags::INTER_NEAREST as isize,
        NearestExact = InterpolationFlags::INTER_NEAREST_EXACT as isize,
    }
    
    
    impl Default for InterpolationConfig {
        fn default() -> Self {
            Self::Lanczos4
        }
    }

    #[derive(Deserialize)]
    #[serde(tag="type")]
    pub enum FrameSizeConfigConfig {
        Static {width: i32, height: i32},
        IResize { width: i32, height: i32, interpolation: InterpolationConfig },
        Resize { width: i32, height: i32 },
        Derive {},
        IDeriveResize { interpolation: InterpolationConfig },
        DeriveResize {},
    }
    
    
    #[derive(Deserialize)]
    #[serde(default)]
    pub struct VideoFileWriterConfig {
        fourcc: [char; 4],
        fps: f64,
        frame_size: FrameSizeConfigConfig,
        api_preference: Option<i32>,
        is_color: bool,
    }


    impl Default for VideoFileWriterConfig {
        fn default() -> Self {
            Self {
                fourcc: ['D', 'I', 'V', 'X'],
                fps: 64.,
                frame_size: FrameSizeConfigConfig::DeriveResize {},
                api_preference: None,
                is_color: true,
            }
        }
    }


    impl Into<VideoFileWriter> for VideoFileWriterConfig {
        fn into(self) -> VideoFileWriter {
            VideoFileWriter::new(
                // TODO: conversion step should happen during parsing
                VideoWriter::fourcc(self.fourcc[0], self.fourcc[1], self.fourcc[2], self.fourcc[3]).unwrap(),
                FPSConfig::Static(self.fps),
                match self.frame_size {
                    FrameSizeConfigConfig::Static {width, height} => {
                        FrameSizeConfig::Static(Size::new(width, height))
                    }
                    FrameSizeConfigConfig::IResize {width, height, interpolation} => {
                        FrameSizeConfig::Resize(Size::new(width, height), interpolation as i32)
                    }
                    FrameSizeConfigConfig::Resize {width, height} => {
                        FrameSizeConfig::Resize(Size::new(width, height), InterpolationFlags::INTER_LANCZOS4 as i32)
                    }
                    FrameSizeConfigConfig::Derive {} => { FrameSizeConfig::Derive }
                    FrameSizeConfigConfig::IDeriveResize {interpolation} => {
                        FrameSizeConfig::DeriveResize(interpolation as i32)
                    }
                    FrameSizeConfigConfig::DeriveResize {} => {
                        FrameSizeConfig::DeriveResize(InterpolationFlags::INTER_LANCZOS4 as i32)
                    }
                },
                self.api_preference,
                self.is_color,
            )
        }
    }
}