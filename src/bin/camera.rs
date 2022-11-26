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
    let mut camera = prepare_camera()?;
    let mut motiondetect = configure()?.create(sender);
    let mut camera_running = true;

    loop {
        match receiver.try_recv() {
            Ok(Signal::StopCamera) => {
                info!("Stopping Camera");
                camera_running = false;
            },
            Ok(Signal::StartCamera) => {
                info!("(Re)starting Camera");
                camera_running = true
            },
            Ok(_) | Err(_) => {},
        }

        if !camera_running {
            continue
        }

        let mut frame = read_camera(&mut camera)?;


        motiondetect = motiondetect.new_frame(&frame)?;

        imshow("Camera", &frame);

        let key = wait_key(1)?;
        if key == 113 {
            break;
        }

    }

    Ok(())
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
