use serenity::model::channel::Message;
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

#[derive(Debug, Error)]
pub enum MsgCommandError {
    #[error("The message command is invalid: {context}")]
    InvalidMessageParameter { context: String },
}
