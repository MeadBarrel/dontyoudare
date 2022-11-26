use std::sync::{Arc, Mutex};
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
use tokio;

use super::signals::*;

type Shared<T> = Arc<Mutex<T>>;


#[derive(BotCommands, PartialEq, Debug)]
#[command(rename_rule="lowercase", parse_with="split")]
enum Command {
    StopCamera,
    StartCamera
}


pub fn run(sender: Sender, receiver: Receiver) -> Result<()>
{
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    //thread::spawn(move || stupid_thread(s));
    rt.block_on(start_bot(sender, receiver))?;
    Ok(())
}


pub async fn start_bot(sender: Sender, receiver: Receiver) -> Result<()> {
    let mut chat_id: ChatId = ChatId( std::env::var("CHAT_ID")?.parse()? ) ;

    let bot = Arc::new(Bot::from_env());

    let handler = dptree::entry().branch(
        Update::filter_message().endpoint(handle_commands)
    );

    // tokio::spawn(stupid_loop(bot.clone(), rec.clone()));

    Dispatcher::builder(bot.clone(), handler )
        .dependencies(deps![sender, chat_id])
        .build()
        .dispatch()
        .await;

    Ok(())
}


async fn handle_commands(bot: Arc<Bot>, msg: Message, sender: Sender, chat_id: ChatId) -> Result<()> {

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