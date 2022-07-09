use discord_rss_notifier::config::EnvConfig;
use discord_rss_notifier::message_handler::Handler;
use dotenv::dotenv;
use serenity::prelude::{Client, GatewayIntents};

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
        eprintln!("Client error: {:?}", why);
    }
}
