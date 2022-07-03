use serenity::model::channel::Message;
use serenity::prelude::Context;
use thiserror::Error;

/// The type of command to issue when a "!cmd" meessage is received
pub enum MsgCommand {
    /// Lists all the RSS subscriptions for the channels
    ListRssSubscriptions,
    /// Subscribe to an RSS Feed
    SubscribeToRssFeed { feed_url: String },
}

impl MsgCommand {
    pub fn new(msg: &Message) -> Result<Option<Self>, MsgCommandError> {
        let message_split: Vec<&str> = msg.content.split(" ").collect();
        match message_split[0] {
            "!rss_list" => Ok(Some(Self::ListRssSubscriptions)),
            "!rss_sub" => {
                if let Some(p) = message_split.get(1) {
                    // TODO: this url should be validated
                    return Ok(Some(Self::SubscribeToRssFeed {
                        feed_url: p.to_string(),
                    }));
                }
                Err(MsgCommandError::InvalidMessageParameter {
                    context: "rss_sub command requires 1 argument, 0 were given".to_string(),
                })
            }
            _ => Ok(None),
        }
    }
}

pub async fn list_channel_subscriptions(msg: Message, ctx: Context) {
    // TODO: Actually get the subscriptions
    // let subscriptions = get_subscriptions_for_channel(*msg.channel_id.as_u64());
    let subscripitions = vec!["a", "b", "c"];
    let formatted_subscriptions = format_subscriptions(subscripitions);
    if let Err(why) = msg.channel_id.say(&ctx.http, formatted_subscriptions).await {
        eprintln!("Error listing rss feeds to channel: {}", why);
    }
}

pub async fn subscribe_channel_to_feed(feed_url: &str, msg: Message, ctx: Context) {
    // TODO: Actually get the subscriptions
    if let Err(why) = msg
        .channel_id
        .say(&ctx.http, format!("Subscribing this channel to {feed_url}"))
        .await
    {
        eprintln!("Failed subscribing to channel: {}", why);
    }
}

/// Returns a string formatted for a discord message
fn format_subscriptions(subscriptions: Vec<&str>) -> String {
    // TODO: format so each subscription is on its own line
    format!("This channel is subscribed to the following RSS feeds: {subscriptions:?}")
}
#[derive(Debug, Error)]
pub enum MsgCommandError {
    #[error("The message command is invalid: {context}")]
    InvalidMessageParameter { context: String },
}
