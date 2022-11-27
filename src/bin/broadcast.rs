pub struct Broadcast<T> where T: Clone+Send+Sync+'static {
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

    pub fn run_loop(&self) -> anyhow::Result<()> {
        loop {
            self.recv()?;
        }
    }

    pub fn recv(&self) -> anyhow::Result<()> {
        println!("RECEIVING");
        let msg = self.receiver.recv()?;
        self.send(msg)?;
        Ok(())
    }

    pub fn send(&self, msg: T) -> anyhow::Result<()> {
        for sender in &self.subscribers {
            sender.send(msg.clone())?;
        }
        Ok(())
    }
}
