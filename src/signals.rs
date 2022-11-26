use crossbeam_channel;


pub type Sender = crossbeam_channel::Sender<Signal>;
pub type Receiver = crossbeam_channel::Receiver<Signal>;


#[derive(Clone)]
pub enum Signal {
    StartCamera,
    StopCamera,
    MotionCaptureStarted,
    MotionCaptured(String),
}