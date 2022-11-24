use anyhow::Result;
use opencv::{
    prelude::*,
    videoio,
    highgui,
};

use ropencv::cv::*;
use ropencv::camera::*;


#[cfg(feature = "toml_config")]
use toml;
#[cfg(feature = "toml_config")]
use std::fs;
use std::rc::Rc;
use std::time::Duration;
use opencv::highgui::imshow;
use opencv::imgproc::InterpolationFlags;
use opencv::videoio::VideoWriter;
use ropencv::camera::handler::Handler;
#[cfg(feature = "toml_config")]
use ropencv::config::config::MatDiffConfig;
use ropencv::camera::motion::motion::MotionDetect;
use ropencv::camera::motion::writer::Writer;
use ropencv::camera::motion::state::StatesConfig;



const MAT1: i32 = 1;
const MAT2: i32 = 2;
const DIFF: i32 = 4;
const DILATE: i32 = 8;
const THRESHOLD: i32 = 16;
const CONTOURS: i32 = 32;

const SHOW_FRAMES: i32 = DIFF | DILATE | THRESHOLD | CONTOURS;

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
fn init_matdiff() -> Result<MatDiff> {
    Ok(MatDiff::default())
}


#[cfg(feature = "toml_config")]
fn init_matdiff() -> Result<MatDiff> {
    let config_toml = fs::read_to_string("config.toml")?;
    let config: MatDiffConfig = toml::from_str(&config_toml)?;
    Ok(config.into())
}


fn main() -> Result<()> {
    let mut cam = prepare_camera()?;
    let vf_writer = VideoFileDirWriter::new(
        VideoFileWriter::default(),
        "%Y-%m-%d-%H-%M-%S.avi",
        "output",
    );
    let diff = MatDiff::default();
    let config = Rc::new(StatesConfig {
        writer: Writer::new(vf_writer),
        min_video_duration: Duration::from_secs(3),
        max_video_duration: Duration::from_secs(15),
        max_idle_gap: Duration::from_secs(3)
    });
    let mut md = MotionDetect::new(diff, config.clone());

    loop {
        let mut frame = read_camera(&mut cam)?;

        md.new_frame(&frame)?;

        imshow("frame", &frame)?;

        let key = highgui::wait_key(1)?;
        if key == 113 {
            break;
        }
    }

    Ok(())
}