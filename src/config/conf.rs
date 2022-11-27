use opencv::videoio::VideoWriter;
use serde::Deserialize;
use crate::config::deserialize_fourcc;


#[derive(Deserialize)]
#[serde(default)]
pub struct DiffConfig {
    // Preprocessing blur radius
    // # note: must be an odd number
    pub blur_radius: i32,
    pub blug_sigma: f64,

    // The radius at which pixels that are close together will be connected
    pub dilate_radius: i32,
    pub dilate_iterations: i32,

    // Minimum area of detected motion, in pixels
    pub sensitivity: i32,

    // How much pixels must change before being detected
    pub threshold: i32,

    // Minimal video duration, in seconds
    pub min_video_duration: u64,

    // Maximum video duration, in seconds
    pub max_video_duration: u64,

    // Maximum allowed gap between motion episodes (without interrupting recording), in seconds
    pub max_idle_gap: u64,

    pub output: OutputFileConfig,
}


impl Default for DiffConfig {
    fn default() -> Self {
        Self {
            blur_radius: 3,
            blug_sigma: 3.5,
            dilate_radius: 6,
            dilate_iterations: 4,
            sensitivity: 4000,
            threshold: 6,
            min_video_duration: 2,
            max_video_duration: 15,
            max_idle_gap: 2,
            output: OutputFileConfig::default()
        }
    }
}


#[derive(Deserialize)]
#[serde(default)]
pub struct OutputFileConfig {
    #[serde(deserialize_with="deserialize_fourcc")]
    pub fourcc: i32,

    pub fps: f64,

    // result video filename format
    pub result_filename_format: String,

    // a folder for resulting video files
    pub result_folder: String
}


impl Default for OutputFileConfig {
    fn default() -> Self {
        Self {
            fourcc: VideoWriter::fourcc('m', 'p', 'v', '4').unwrap(),
            fps: 24.,
            result_filename_format: "%Y-%m-%d-%H-%M-%S.mp4".to_owned(),
            result_folder: "output".to_owned()
        }
    }
}