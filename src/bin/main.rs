use std::fs::File;
use std::thread;
use anyhow::Result;
use crossbeam_channel::unbounded;
use crossbeam_channel;
use log::*;
use simplelog::*;
use ropencv::broadcast::Broadcast;

use ropencv::signals::*;
use ropencv::cam;
use ropencv::telegram;

fn main() {
    init_logger("log.log").unwrap();

    let (sender, receiver) = unbounded();
    let mut broadcast = Broadcast::new(receiver);

    let camera_thread = run_camera(sender.clone(), broadcast.subscribe());

    let telegram_thread = run_telegram(sender.clone(), broadcast.subscribe());

    thread::spawn(move || broadcast.run_loop());

    camera_thread.join().expect("Camera thread has panicked").unwrap();

    telegram_thread.join().expect("Telegram thread has panicked").unwrap();
}


fn run_camera(sender: Sender, receiver: Receiver) -> thread::JoinHandle<Result<()>> {
    thread::spawn(|| { cam::run(sender, receiver) })
}

fn run_telegram(sender: Sender, receiver: Receiver) -> thread::JoinHandle<Result<()>> {
    info!("Starting telegram bot");
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
