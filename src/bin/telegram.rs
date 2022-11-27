use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use anyhow::Result;
use log::info;
use teloxide::{
    utils::command::BotCommands,
    Bot,
    dptree,
    dptree::deps,
    prelude::*,
    types::Update,
    dispatching::UpdateFilterExt,
};
use teloxide::types::InputFile;
use tokio;
use tokio::time::sleep;


use ropencv::signals::*;

type Shared<T> = Arc<Mutex<T>>;


#[derive(BotCommands, PartialEq, Debug)]
#[command(rename_rule="lowercase", parse_with="split")]
enum Command {
    StopCamera,
    StartCamera
}


pub fn run(sender: Sender, receiver: Receiver) -> Result<()>
{
    let rt = tokio::runtime::Runtime::new().unwrap();
    //thread::spawn(move || stupid_thread(s));
    rt.block_on(start_bot(sender, receiver))?;
    Ok(())
}


pub async fn start_bot(sender: Sender, receiver: Receiver) -> Result<()> {
    let chat_id: ChatId = ChatId( std::env::var("CHAT_ID")?.parse()? ) ;

    let bot = Bot::from_env();

    let handler = dptree::entry().branch(
        Update::filter_message().endpoint(handle_commands)
    );

    tokio::spawn(notificator_loop(bot.clone(), receiver.clone(), chat_id));

    Dispatcher::builder(bot.clone(), handler )
        .dependencies(deps![sender, chat_id])
        .build()
        .dispatch()
        .await;

    Ok(())
}


async fn notificator_loop(bot: Bot, receiver: Receiver, chat_id: ChatId) -> Result<()> {
    loop {
        sleep(Duration::from_secs(1)).await;
        match receiver.try_recv() {
            Ok(Signal::MotionCaptured(path)) => {
                info!("Captured motion at {:?}", path);
                bot.send_message(chat_id, "Detected motion".to_string()).await?;
                let path_buf = PathBuf::from_str(&path)?;
                bot.send_video(chat_id, InputFile::file(path_buf)).await?;
            }
            Ok(_) | Err(_) => {}
        };
    }
}


async fn handle_commands(bot: Bot, msg: Message, sender: Sender, chat_id: ChatId) -> Result<()> {

    // Make sure that only our chat is supported
    if chat_id != msg.chat.id {
        info!("Unauthorized attempt from {:?}", msg.chat);
        return Ok(());
    }

    let msg_text = msg.text();

    if msg_text == None {
        info!("Received a message but with no text");
        return Ok(());
    }

    info!("{}", msg.text().unwrap());

    let command = Command::parse(msg.text().unwrap(), "Bot");

    match command {
        Ok(Command::StopCamera) => { sender.try_send(Signal::StopCamera)? },
        Ok(Command::StartCamera) => { sender.try_send(Signal::StartCamera)? }
        Err(_) => {}
    }

    Ok(())
}