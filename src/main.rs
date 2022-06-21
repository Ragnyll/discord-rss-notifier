use discord_rss_notifier::config::EnvConfig;

use dotenv::dotenv;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::collections::HashSet;
use tokio::time::{sleep, Duration };

struct Handler {
    db_poll_sec: Duration
}


#[async_trait]
impl EventHandler for Handler {

    /// change this for adding the given channel id to the bot
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Boss Please!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    /// When the bot joins the channel do these things
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{}: bot joined the channel", ready.user.name);
        loop {
            sleep(self.db_poll_sec).await;
            for c_id in get_channel_ids_to_send_to() {
                if let Err(why) = ChannelId(c_id)
                    .say(&ctx.http, "Can somebody gibe da pusi pls?")
                    .await
                {
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

fn get_channel_ids_to_send_to() -> HashSet<u64> {
    HashSet::from_iter(vec![988182924581036032])
}

#[tokio::main]
async fn main() {
    let local_build_option = std::env::args().nth(1);
    if  local_build_option.is_some() && local_build_option.unwrap() == "--local" {
        println!("running in local mode, receiving env vars from dotenv");
        dotenv().ok();
    }

    let env_conf = EnvConfig::new();
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&env_conf.discord_token, intents)
        .event_handler(Handler {db_poll_sec: env_conf.db_poll_sec })
        .await
        .expect("Err creating client");

    if let Err(why) = client.start_autosharded().await {
        println!("Client error: {:?}", why);
    }
}
