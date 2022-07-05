use diesel::prelude::*;
use std::collections::HashSet;
use thiserror::Error;

pub struct QueryManager {
    // A connection to the sqlite database
    connection: SqliteConnection,
}

impl QueryManager {
    /// Creates a new QueryManager connected to the given database
    pub fn new(database_url: &str) -> Result<Self, diesel::ConnectionError> {
        Ok(Self {
            connection: SqliteConnection::establish(database_url)?,
        })
    }

    /// Given the channel id returns all the feeds the channel is Subscribed to
    pub fn get_all_feed_subscriptions(channel_id: u64) -> HashSet<String> {
        todo!("I cant get {channel_id} channel_id's subscriptions yet!");
    }

    /// Subscribes the given channel_id to the given feed_url
    pub fn add_feed_to_channel_subscriptions(
        channel_id: u64,
        feed_url: &str,
    ) -> Result<(), QueryError> {
        todo!("I cant subscribe {channel_id} to feed_url {feed_url} yet");
    }

    /// Gets all the updated feed_item urls for given channel_id
    pub fn get_all_feed_updates(channel_id: u64) -> HashSet<String> {
        todo!("I cant get feed updates for {channel_id} yet");
    }
}

#[derive(Debug, Error)]
pub enum QueryError {}
