use std::fs;
use std::time::Duration;
use toml;
use anyhow::Result;
use log::info;
use opencv::{
    videoio::{VideoCapture, VideoCaptureTrait, CAP_PROP_FRAME_WIDTH, CAP_PROP_FRAME_HEIGHT, CAP_ANY},
    highgui::{imshow, wait_key},
    prelude::Mat,
};
use opencv::core::{BORDER_DEFAULT, BorderTypes, Point, Scalar, Size};
use opencv::imgproc::{InterpolationFlags, MORPH_ELLIPSE, THRESH_BINARY};
use simplelog::Config;

use crate::camera::{Handler, MatDiff, MotionDetect, StatesConfig, Writer};
use crate::cv::*;
use crate::config::DiffConfig;


use crate::signals::*;


pub fn run(sender: Sender, receiver: Receiver) -> Result<()> {
    let camera = prepare_camera()?;
    let motiondetect = configure(sender)?;

    let mut runner = CameraRunner::new(camera, motiondetect);

    loop {
        match receiver.try_recv() {
            Ok(signal) => {
                runner.handle_signal(signal)?;
            }
            Err(_) => { }
        };

        runner = runner.next()?;

        let key = wait_key(1)?;
        if key == 113 {
            break;
        }

    }

    Ok(())
}


pub struct  CameraRunner {
    camera: VideoCapture,
    motiondetect: MotionDetect,

    camera_running: bool,
}


impl CameraRunner {
    pub fn new(camera: VideoCapture, motiondetect: MotionDetect) -> Self {
        Self {
            camera,
            motiondetect,
            camera_running: true
        }
    }

    pub fn next(mut self) -> Result<Self> {
        if !self.camera_running { return Ok(self) }

        let frame = self.read_camera()?;
        self.motiondetect = self.motiondetect.new_frame(&frame)?;

        self.show_frame(&frame)?;

        Ok(self)
    }

    fn show_frame(&self, frame: &Mat) -> Result<()> {
        Ok(imshow("Camera", frame)?)
    }

    fn handle_signal(&mut self, signal: Signal) -> Result<()> {
        match signal {
            Signal::StopCamera => {
                info!("Stopping Camera");
                self.camera_running = false;
            },
            Signal::StartCamera => {
                info!("(Re)starting Camera");
                self.camera_running = true;
            },
            _ => {}
        }
        Ok(())
    }

    fn read_camera(&mut self) -> Result<Mat> {
        read_camera(&mut self.camera)
    }

}


fn configure(sender: Sender) -> Result<MotionDetect> {
    let config_toml = fs::read_to_string("config.toml")?;
    let config: DiffConfig = toml::from_str(&config_toml)?;
    let diff = MatDiff::new(
        GaussianBlur::new(
            Size::new(config.blur_radius, config.blur_radius),
            config.blug_sigma, config.blug_sigma,
            BORDER_DEFAULT,
        ),
        Dilate::new(
            StructuringElement::new(
                MORPH_ELLIPSE,
                Size::new(
                    config.dilate_radius,
                    config.dilate_radius,
                ),
                Point::new(-1, -1),
            ),
            Point::new(-1, -1),
            config.dilate_iterations,
            BorderTypes::BORDER_ISOLATED as i32,
            Scalar::default()

        ),
        Threshold::new(
            config.threshold as f64,
            255.,
            THRESH_BINARY,
        ),
        FindContours::default(),
        config.sensitivity
    );
    let writer = Writer::new(
        VideoFileDirWriter::new(
            VideoFileWriter::new(
                config.output.fourcc,
                FPSConfig::Static(config.output.fps),
                FrameSizeConfig::DeriveResize(InterpolationFlags::INTER_LANCZOS4 as i32),
                None,
                true,
            ),
            &config.output.result_filename_format,
            &config.output.result_folder,
        ),
        sender
    );
    let md = MotionDetect::new(
        diff,
        StatesConfig {
            writer: writer,
            min_video_duration: Duration::from_secs(config.min_video_duration),
            max_video_duration: Duration::from_secs(config.max_video_duration),
            max_idle_gap: Duration::from_secs(config.max_idle_gap),
        }
    );

    Ok(md)
}


fn prepare_camera() -> Result<VideoCapture> {
    let mut cam = VideoCapture::new(0, CAP_ANY)?;
    cam.set(CAP_PROP_FRAME_WIDTH, 640_f64)?;
    cam.set(CAP_PROP_FRAME_HEIGHT, 480_f64)?;
    Ok(cam)
}


fn read_camera(camera: &mut VideoCapture) -> Result<Mat> {
    let mut result = Mat::default();
    camera.read(&mut result)?;
    Ok(result)
}
