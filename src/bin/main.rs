use std::{fs::File, fs};
use std::time::Duration;
use anyhow::Result;
use opencv::{
    prelude::*,
    videoio,
    videoio::{VideoCaptureTrait, VideoCapture},
    highgui::*,
};
use log::*;
use simplelog::*;

use ropencv::{
    camera::*,
    cv::videoio::*,
};

#[cfg(feature = "toml_config")]
use toml;


fn prepare_camera() -> Result<videoio::VideoCapture> {
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    cam.set(videoio::CAP_PROP_FRAME_WIDTH, 640_f64);
    cam.set(videoio::CAP_PROP_FRAME_HEIGHT, 480_f64);
    Ok(cam)
}


fn read_camera(camera: &mut videoio::VideoCapture) -> Result<Mat> {
    let mut result = Mat::default();
    camera.read(&mut result)?;
    Ok(result)
}

#[cfg(not(feature = "toml_config"))]
fn init_configurate() -> Result<MotionDetect> {
    Ok(MotionDetect::default())
}


#[cfg(feature = "toml_config")]
fn configurate() -> Result<MotionDetect> {
    let config_toml = fs::read_to_string("config.toml")?;
    Ok(toml::from_str(&config_toml)?)
}


fn init_logger(filename: &str) -> Result<()> {
    Ok(CombinedLogger::init(
        vec![
            TermLogger::new(
                LevelFilter::Debug,
                Config::default(),
                TerminalMode::Mixed,
                ColorChoice::Auto
            ),
            WriteLogger::new(
                LevelFilter::Info,
                Config::default(),
                File::create(filename)?
            )
        ]
    )?)
}


fn main() -> Result<()> {
    init_logger("log.log");

    info!("Starting");

    let mut cam = prepare_camera()?;
    let mut md = configurate()?;


    loop {
        let mut frame = read_camera(&mut cam)?;

        md = md.new_frame(&frame)?;

        imshow("frame", &frame)?;

        let key = wait_key(1)?;
        if key == 113 {
            break;
        }
    }

    Ok(())
}