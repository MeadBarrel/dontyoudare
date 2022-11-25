pub mod motion;
pub mod writer;
pub mod state;
mod state_watching;
mod state_recording_motion;
mod state_recording_idle;

pub use motion::MotionDetect;
pub use writer::Writer;
pub use state::StatesConfig;