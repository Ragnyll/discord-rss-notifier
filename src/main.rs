use discord_rss_notifier::config::EnvConfig;

use dotenv::dotenv;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::collections::HashSet;
use tokio::time::{sleep, Duration};
use thiserror::Error;

#[derive(Debug, Error)]
enum HandlerError {
    #[allow(dead_code)]
    #[error("unable to subscribe channel to feed")]
    SubscribeError,
    #[allow(dead_code)]
    #[error("unable to receive updates to feed items")]
    UpdateReceiveFailure,
}

struct Handler {
    db_poll_sec: Duration,
    #[allow(dead_code)]
    db_url: String,
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
        if msg.content.starts_with("!rss_sub") {
            let message_split: Vec<&str> = msg.content.split(" ").collect();
            if message_split.len() != 2 {
                if let Err(why) = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        "please subscribe channel to rss feed with the command !rss_sub <feed_url>",
                    )
                    .await
                {
                    println!("Error subscribing to rss feed: {}", why);
                }
            }

            let feed_url = message_split[1];
            if let Ok(_) = self.subscribe_channel_to_feed(*msg.channel_id.as_u64(), feed_url) {
                if let Err(why) = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        format!("I've subscribed this channel to {}", feed_url),
                    )
                    .await
                {
                    println!("Error subscribing to rss feed: {}", why);
                }
            }
        } else if msg.content.starts_with("!rss_list") {
            let subscriptions = self.get_subscriptions_for_channel(*msg.channel_id.as_u64());
            let formatted_subscriptions = self.format_subscriptions(subscriptions);
            if let Err(why) = msg
                .channel_id
                .say(
                    &ctx.http,
                    formatted_subscriptions,
                )
                .await
            {
                println!("Error listing rss feeds to channel: {}", why);
            }
        }
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

#[tokio::main]
async fn main() {
    let local_build_option = std::env::args().nth(1);
    if local_build_option.is_some() && local_build_option.unwrap() == "--local" {
        println!("running in local mode, receiving env vars from dotenv");
        dotenv().ok();
    }

    let env_conf = EnvConfig::new();
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&env_conf.discord_token, intents)
        .event_handler(Handler {
            db_poll_sec: env_conf.db_poll_sec,
            db_url: env_conf.database_url,
        })
        .await
        .expect("Err creating client");

    if let Err(why) = client.start_autosharded().await {
        println!("Client error: {:?}", why);
    }
}
