use std::fs::File;
use std::sync::{Arc, Mutex};
use std::thread;
use anyhow::Result;
use crossbeam_channel::unbounded;
use crossbeam_channel;
use log::*;
use simplelog::*;

use ropencv::signals::*;

mod camera;

#[cfg(feature = "telegram")]
mod telegram;


fn main() {
    // let (sender, receiver) = unbounded();
    // let mut broadcast:Broadcast<String> = Broadcast::new(receiver);
    // let r1 = broadcast.subscribe();
    // let r2 = broadcast.subscribe();
    //
    // broadcast.send("abc".to_string()).unwrap();
    // println!("r1 {}", r1.recv().unwrap());
    // println!("r2 {}", r2.recv().unwrap());
    //
    // return ();
    init_logger("log.log");

    let (sender, receiver) = unbounded();
    let mut broadcast = Broadcast::new(receiver);

    let mut camera_thread = run_camera(sender.clone(), broadcast.subscribe());

    #[cfg(feature = "telegram")]
    let mut telegram_thread = run_telegram(sender.clone(), broadcast.subscribe());

    thread::spawn(move || broadcast.run_loop());

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


struct Broadcast<T> where T: Clone+Send+Sync+'static {
    receiver: crossbeam_channel::Receiver<T>,
    subscribers: Vec<crossbeam_channel::Sender<T>>
}


impl<T> Broadcast<T> where T: Clone+Send+Sync+'static {
    pub fn new(receiver: crossbeam_channel::Receiver<T>) -> Self {
        Self {
            receiver,
            subscribers: Vec::new()
        }
    }

    pub fn subscribe(&mut self) -> crossbeam_channel::Receiver<T> {
        let (sender, receiver) = crossbeam_channel::unbounded();
        self.subscribers.push(sender);
        receiver
    }

    pub fn run_loop(&self) -> Result<()> {
        loop {
            self.recv()?;
        }
    }

    pub fn recv(&self) -> Result<()> {
        let msg = self.receiver.recv()?;
        self.send(msg)?;
        Ok(())
    }

    pub fn send(&self, msg: T) -> Result<()> {
        for sender in &self.subscribers {
            sender.send(msg.clone())?;
        }
        Ok(())
    }
}
