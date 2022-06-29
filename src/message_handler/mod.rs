use crate::message_handler::message_commands::ProcessMessage;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::collections::HashSet;
use thiserror::Error;
use tokio::time::{sleep, Duration};

pub mod message_commands;

/// Handler for Discord Client actions
pub struct Handler {
    pub db_poll_sec: Duration,
    #[allow(dead_code)]
    pub db_url: String,
}

impl Handler {
    /// gets all the unread freed items for the specified channel
    fn get_feed_item_updates_for_channel(&self) -> Result<HashSet<String>, HandlerError> {
        Ok(HashSet::new())
    }

    /// subscribes the channel to rss feed_url
    fn subscribe_channel_to_feed(
        &self,
        channel_id: u64,
        feed_url: &str,
    ) -> Result<(), HandlerError> {
        Ok(())
    }

    fn get_channel_ids_to_send_to(&self) -> HashSet<u64> {
        HashSet::from_iter(vec![])
    }

    fn get_subscriptions_for_channel(&self, channel_id: u64) -> HashSet<String> {
        todo!("I cant get the subscrciptions for this {channel_id} yet");
    }

    fn format_subscriptions(&self, subscriptions: HashSet<String>) -> String {
        todo!("I cant format subscriptions yet");
    }
}

#[async_trait]
impl EventHandler for Handler {
    /// change this for adding the given channel id to the bot
    async fn message(&self, ctx: Context, msg: Message) {
        let msg_cmd = message_commands::new_message_command(&msg);
        match msg_cmd {
            Ok(c) => {
                match c {
                    Some(c) => {
                        // TODO: no unwrap
                        // let c = &c as &dyn ProcessMessage;
                        let _ = c.process();
                    },
                    None => (),
                }

            }
            Err(e) => {

        // if let Err(why) = msg.channel_id.say(&ctx.http, context).await {
            // log::error!("Error handling client command: {why}");
        // }
                // msg.channel_id.say(&ctx.http, e).await;
                log::error!("Error processing message_type: {:?}", e);
            },
        };
    }

    /// Starts up when the Handler is ready. This starts the polling loop for rss updates
    async fn ready(&self, ctx: Context, _: Ready) {
        loop {
            sleep(self.db_poll_sec).await;
            for c_id in self.get_channel_ids_to_send_to() {
                // check db for rss feed updates
                // push update to the channel
                if let Err(why) = ChannelId(c_id).say(&ctx.http, "Every few seconds").await {
                    eprintln!("error sending update {:?}", why);
                }
            }
        }

        // This works ^^^ but do the following
        // create a db for all rss items. set a ttl on them for 1 week (easily done with cosmos db)
        // Async sleep
        // on every wakeup check a db for all channel_ids
        // for every channel ids retrieve their new rss feeds, and publish them to the channel
    }
}

#[derive(Debug, Error)]
pub enum HandlerError {
    #[allow(dead_code)]
    #[error("unable to subscribe channel to feed")]
    SubscribeError,
    #[allow(dead_code)]
    #[error("unable to receive updates to feed items")]
    UpdateReceiveFailure,
}
