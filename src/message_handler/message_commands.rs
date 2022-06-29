use async_trait::async_trait;
use serenity::model::channel::Message;
use thiserror::Error;

/// The to level way of generating new commands for the
pub fn new_message_command(
    msg: &Message,
) -> Result<Option<Box<dyn ProcessMessage>>, MsgCommandError> {
    let message_split: Vec<&str> = msg.content.split(" ").collect();
    match message_split[0] {
        "!rss_list" => Ok(Some(Box::new(RssListCommand {}))),
        "!rss_sub" => {
            if let Some(p) = message_split.get(1) {
                // TODO: this url should be validated
                return Ok(Some(Box::new(RssSubscribeCommand {
                    feed_url: p.to_string(),
                })));
            }
            Err(MsgCommandError::InvalidMessageParameter {
                context: "rss_sub command requires 1 argument, 0 were given".to_string(),
            })
        }
        _ => Ok(None),
    }
}
// match msg_cmd {
    // // The command is valid process it
    // Ok(cmd) => match cmd {
        // Some(message_commands::MsgCommand::ListRssSubscriptions) => {
            // let subscriptions =
                // self.get_subscriptions_for_channel(*msg.channel_id.as_u64());
            // let formatted_subscriptions = self.format_subscriptions(subscriptions);
            // if let Err(why) = msg.channel_id.say(&ctx.http, formatted_subscriptions).await {
                // println!("Error listing rss feeds to channel: {}", why);
            // }
        // }
        // Some(message_commands::MsgCommand::SubscribeToRssFeed { feed_url }) => {
            // if let Ok(_) =
                // self.subscribe_channel_to_feed(*msg.channel_id.as_u64(), &feed_url)
            // {
                // if let Err(why) = msg
                    // .channel_id
                    // .say(
                        // &ctx.http,
                        // format!("I've subscribed this channel to {}", feed_url),
                    // )
                    // .await
                // {
                    // println!("Error subscribing to rss feed: {}", why);
                // }
            // }
        // }
        // None => (),
    // },
    // // The command is invalid. The bot should respond then move on
    // Err(context) => {
        // if let Err(why) = msg.channel_id.say(&ctx.http, context).await {
            // log::error!("Error handling client command: {why}");
        // }
    // }
// }

/// Messages that can be processed by the discord bot
pub trait ProcessMessage {
    /// The behaviour that should be ran by the command
    fn process(&self) -> Result<(), MessageProcessError>;
}

/// Lists all Rss Subscriptions that a channel has
pub struct RssListCommand {}

impl ProcessMessage for RssListCommand {
    fn process(&self) -> Result<(), MessageProcessError> {
        todo!("implement process message");
    }
}

/// Subscribes to a new RSS Feed
pub struct RssSubscribeCommand {
    /// The rss feed url for the command to attempt to subscribe to.
    feed_url: String,
}

impl ProcessMessage for RssSubscribeCommand {
    fn process(&self) -> Result<(), MessageProcessError> {
        todo!("implement process message");
    }
}

#[derive(Debug, Error)]
pub enum MsgCommandError {
    #[error("The message command is invalid: {context}")]
    InvalidMessageParameter { context: String },
}

#[derive(Debug, Error)]
pub enum MessageProcessError {
    #[error("The message failed to process: {context}")]
    ProcessMessageError { context: String },
}
