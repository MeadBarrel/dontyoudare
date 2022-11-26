use std::fs::File;
use std::sync::{Arc, Mutex};
use std::thread;
use anyhow::Result;
use crossbeam_channel::unbounded;
use log::*;
use simplelog::*;

use signals::*;

mod camera;
mod signals;

#[cfg(feature = "telegram")]
mod telegram;


fn main() {
    init_logger("log.log");

    let (sender, receiver) = unbounded();

    let mut camera_thread = run_camera(sender.clone(), receiver.clone());

    #[cfg(feature = "telegram")]
    let mut telegram_thread = run_telegram(sender.clone(), receiver.clone());


    camera_thread.join().expect("Camera thread has panicked").unwrap();

    #[cfg(feature = "telegram")]
    telegram_thread.join().expect("Telegram thread has panicked").unwrap();
}


fn run_camera(sender: Sender, receiver: Receiver) -> thread::JoinHandle<Result<()>> {
    thread::spawn(|| { camera::run(sender, receiver) })
}

#[cfg(feature = "telegram")]
fn run_telegram(sender: Sender, receiver: Receiver) -> thread::JoinHandle<Result<()>> {
    thread::spawn(|| { telegram::run(sender, receiver) })
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