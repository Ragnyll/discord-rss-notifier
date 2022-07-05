use std::env;
use tokio::time::Duration;

// Env var names
/// A token for interacting with the discord bot api
const DISCORD_TOKEN: &str = "DISCORD_TOKEN";
const DB_POLL_SEC: &str = "DB_POLL_SEC";
const DATABASE_URL: &str = "DATABASE_URL";

// Defaults

/// The amount of time the bot will sleep before checking for new rss feeds to publish to channels.
const DEFAULT_SLEEP_TIME: Duration = Duration::from_secs(900);

/// A config object derived from env vars
pub struct EnvConfig {
    pub discord_token: String,
    pub db_poll_sec: Duration,
    pub database_url: String,
}

impl Default for EnvConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl EnvConfig {
    pub fn new() -> Self {
        Self {
            discord_token: env::var(DISCORD_TOKEN).expect("{DISCORD_TOKEN} env var required"),
            db_poll_sec: {
                if let Ok(s) = env::var(DB_POLL_SEC) {
                    Duration::from_secs(
                        s.parse::<u64>()
                            .expect("Unable to parse DB_POLL_SEC to u64"),
                    )
                } else {
                    DEFAULT_SLEEP_TIME
                }
            },
            database_url: env::var(DATABASE_URL).expect("{DATABASE_URL} env var required"),
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_test() {
        assert!(true)
    }
}
