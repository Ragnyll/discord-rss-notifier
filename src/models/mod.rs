/// The models shared for database management and feed ser/de
use std::collections::HashSet;

/// TODO: this would probably be better represented with a trait object. Improvement for later
/// Represents all the channels subscribed to a given feed
#[derive(Debug)]
pub struct FeedSubscription {
    /// A unique identifier for a feed subscription
    pub id: u64,
    /// The url from which to read the feed from
    pub feed_url: String,
    /// The base url from which to build links to the individual feed_items
    pub base_url: String,
    /// The set of channels subscribed to this Feed
    pub channel_ids: HashSet<u64>,
    /// Represents the xml spec of the FeedItems in this Feed
    pub feed_type: FeedType
}

/// Represents an individual feed item
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct FeedItem {
    /// A Foreign Key link to a FeedSubscription
    pub feed_id: u64,
    /// The full link to the feed item
    pub link: String
}

/// The various types of feeds that can be inserted into the database.
#[derive(Debug, Hash)]
pub enum FeedType {
    Rss,
    Atom,
}
