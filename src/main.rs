use std::fs::OpenOptions;
use std::io::{self, Write};
use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, UpdateHandler},
    prelude::*,
    types::{MediaKind, MediaText, Message, MessageEntityKind, MessageId, MessageKind, MediaPhoto, ChatId, UserId},
    utils::command::BotCommands,
    net::Download,
};
use tokio::fs;

use reqwest::header::*;
use url::Url;

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

const BRAIN_LOCATION: &str = "brain.md";

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
}

// commands

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "displaly Help this text.")]
    Help,
    #[command(description = "start stufftuff")]
    Start,
    #[command(description = "cancel stuff")]
    Cancel,
    #[command(description = "flower stuff")]
    Flower,
}

#[derive(Clone)]
struct ConfigParameters {
    bot_maintainer: UserId,
}

const PARAMETERS: ConfigParameters = ConfigParameters {
    bot_maintainer: UserId(51739298), // Paste your ID to run this bot.
};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting dialogue bot...");

    let bot = Bot::from_env();

    Dispatcher::builder(bot, update_handler())
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

fn update_handler() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Start].endpoint(start))
        .branch(case![Command::Help].endpoint(help))
        .branch(case![Command::Flower].endpoint(flower))
        .branch(case![Command::Cancel].endpoint(cancel));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::Start].endpoint(handle_message));

    dialogue::enter::<Update, InMemStorage<State>, State, _>().branch(message_handler)
}

async fn help(bot: Bot, msg: Message) -> HandlerResult {
    let help_message = format!(
        "help has been invoked, your user id is {}",
        msg.from().expect("User should have id").id
    );
    bot.send_message(msg.chat.id, help_message).await?;
    Ok(())
}

async fn cancel(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "cancel has been invoked")
        .await?;
    Ok(())
}

async fn flower(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "hi").await?;
    Ok(())
}

async fn start(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "This is the start").await?;
    Ok(())
}

async fn handle_message(bot: Bot, _dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let hm = bot.send_message(msg.chat.id.clone(), "This is the handle_message")
        .reply_to_message_id(msg.id)
        .await?;

    let _is_owner = |msg: &Message| {
        msg.from()
            .map(|user| {
                // let reply = format!("Your id is {}", user.id);
                user.id == PARAMETERS.bot_maintainer
            })
            .unwrap_or_default()
    };

    if !_is_owner(&msg) {
        bot.send_message(msg.chat.id, "Estoy para contar las veces que se llama la estupidez humana!")
            .reply_to_message_id(msg.id)
            .await?;
        return Ok(());
    }
    match msg.kind {
        MessageKind::Common(chat) => {
            match chat.media_kind {
                MediaKind::Text(content) => {
                    handle_text_content(bot.clone(), msg.chat.id, msg.id, Some(content)).await?;
                }
                _ => {
                    bot.send_message(msg.chat.id, "Media::Kind Type not implemented")
                        .reply_to_message_id(msg.id)
                        .await?;
                    log::debug!("{:#?} not implemented", chat);
                    // log::debug!("{:#?} not implemented", msg);
                } //todo!(), // todo for media_kind
            }
        }
        // _ => todo!(), //todo for msg kind
        _ => {
            bot.send_message(msg.chat.id, "MessageKind not implemented")
                .reply_to_message_id(msg.id)
                .await?;
        } //todo!(), // todo for media_kind
    };
    // }
    let msg_id = hm.id;
    log::debug!("message id: {:#?}", &msg_id);
    bot.delete_message(msg.chat.id, msg_id).await?;
    Ok(())
}


async fn handle_text_content(
    bot: Bot,
    chat_id: ChatId,
    message_id: MessageId,
    message_text: Option<MediaText>,
) -> HandlerResult {
    bot.send_message(chat_id, "Got text message")
        .reply_to_message_id(message_id)
        .await?;

    let content = message_text.unwrap();
    log::info!("text: {}", content.text);
    log::debug!("object: {:#?}", content.text);

    bot.send_message(chat_id, "Se suma pero no se resta").await?;

    log::debug!("found {}", content.entities.len());
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    //
    // * “One good test is worth a thousand expert opinions.” \n
    // * – Wernher von Braun @ twitter https://test.com
    // *
    // * thght.works/3ghJZ9t => problem
    // thread 'tokio-runtime-worker' panicked at 'failed trying to parse >: https://thght.works/3vZX6<: RelativeUrlWithoutBase', telegram/src/main.rs:219:40
    //
    // // This has to be converted to a json object
	// DEBUG telegram                          > object: MediaText {
	//    text: "Santiago Zarate, [Jul 8, 2023 at 20:32]\nhttps://www.reddit.com/user/Remarkable-Goat-973/",
	//    entities: [
	//        MessageEntity {
	//            kind: Url,
	//            offset: 40,
	//            length: 48,
	//        },
	//    ],
	//}
    // * */
}
