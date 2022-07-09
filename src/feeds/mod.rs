/// The feeds module is in charge of determining the feeds type (RSS or Atom) deserializing it into an object and providing bindings for inserting it into a db.

/// Not all feeds are RSS. What people call RSS actually usually includes atom feeds, which are far
/// more common these these days. Both are valid and need to be handled.
use crate::db::{QueryManager, QueryError};
use std::error::Error;
use std::sync::Mutex;

use atom_syndication as atom;

/// Updates the db with all the new feed items
pub async fn update_feed_items(query_manager: Mutex<QueryManager>) -> Result<(), QueryError> {
    let qm = query_manager.lock().unwrap();
    // All the sub feeds for all channels.
    let all_feed_subs = qm.get_all_subscriptions()?;

    for sub in all_feed_subs {
        match determine_feed_type(&sub) {
            Rss => (),
            Atom => (),
        }
    }

    Ok(())
}

enum FeedType {
    Rss,
    Atom,
}

async fn example_feed() -> Result<atom::Feed, Box<dyn Error>> {
    let content = reqwest::get("https://ragnyll.github.io/feed.xml")
        .await?
        .bytes()
        .await?;
    let feed = atom::Feed::read_from(&content[..]).unwrap();
    println!("{:?}", feed.base());

    Ok(feed)
}

fn determine_feed_type(feed: &str) -> FeedType {
    todo!("i cant yet determine_feed_type");
}
