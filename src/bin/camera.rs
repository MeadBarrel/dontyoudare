use std::fs;
use toml;
use anyhow::Result;
use log::info;
use opencv::{
    videoio::{VideoCapture, VideoCaptureTrait, CAP_PROP_FRAME_WIDTH, CAP_PROP_FRAME_HEIGHT, CAP_ANY},
    highgui::{imshow, wait_key},
    prelude::Mat,
};

use ropencv::camera::{Handler, MotionDetect};
use ropencv::camera::motion::motion::MotionDetectConfig;


use ropencv::signals::*;


pub fn run(sender: Sender, receiver: Receiver) -> Result<()> {
    let camera = prepare_camera()?;
    let motiondetect = configure()?.create(sender);

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

        let mut frame = self.read_camera()?;
        self.motiondetect = self.motiondetect.new_frame(&frame)?;

        self.show_frame(&frame)?;

        Ok(self)
    }

    fn show_frame(&self, frame: &Mat) -> Result<()> {
        Ok(imshow("Camera", frame)?)
    }

    fn handle_signal(&mut self, signal: Signal) -> Result<()> {
        info!("HANDLING SIGNAL");
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


fn configure() -> Result<MotionDetectConfig> {
    let config_toml = fs::read_to_string("config.toml")?;
    Ok(toml::from_str(&config_toml)?)
}


fn prepare_camera() -> Result<VideoCapture> {
    let mut cam = VideoCapture::new(0, CAP_ANY)?;
    cam.set(CAP_PROP_FRAME_WIDTH, 640_f64);
    cam.set(CAP_PROP_FRAME_HEIGHT, 480_f64);
    Ok(cam)
}


fn read_camera(camera: &mut VideoCapture) -> Result<Mat> {
    let mut result = Mat::default();
    camera.read(&mut result)?;
    Ok(result)
}
