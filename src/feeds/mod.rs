/// The feeds module is in charge of determining the feeds type (RSS or Atom) deserializing it into an object and providing bindings for inserting it into a db.

/// Not all feeds are RSS. What people call RSS actually usually includes atom feeds, which are far
/// more common these these days. Both are valid and need to be handled.
use bytes::Bytes;
use crate::db::QueryManager;
use crate::models::{FeedItem, FeedSubscription, FeedType};
use std::collections::HashSet;
use std::error::Error;
use std::sync::Mutex;

use atom_syndication as atom;

/// Updates the db with all the new feed items
pub async fn update_feed_items(query_manager: Mutex<QueryManager>) -> Result<(), Box<dyn Error>> {
    let qm = query_manager.lock().unwrap();
    // All the sub feeds for all channels.
    let all_feed_subs = qm.get_all_subscriptions()?;

    for sub in all_feed_subs {
        update_feed_items_for_subscription(&sub, &qm)?;
    }

    Ok(())
}

// TODO: i could proably be smarter around the blocking/async apis, but I'm kinda lazy rn
fn update_feed_items_for_subscription(
    subscription: &FeedSubscription,
    query_manager: &QueryManager,
) -> Result<(), Box<dyn Error>> {
    let feed = reqwest::blocking::get(&subscription.feed_url)?.bytes();
    let feed_items = match feed {
        Ok(f) => match subscription.feed_type {
            // TODO: update get the right feed_id
            FeedType::Rss => get_rss_feed_items(subscription.id, f)?,
            FeedType::Atom => get_atom_feed_items(&subscription.base_url, subscription.id, f)?,
        },
        Err(e) => {
            eprintln!(
                "I was unable to retrieve feed updates for {:?}: {:?}",
                subscription.feed_url, e
            );

            HashSet::new()
        }
    };
    println!("feed_items: {feed_items:?}");
    query_manager.update_feed_items(feed_items)?;

    Ok(())
}

fn get_atom_feed_items(feed_base_url: &str, feed_id: u64, feed: Bytes) -> Result<HashSet<FeedItem>, Box<dyn Error>> {
    let feed = atom::Feed::read_from(&feed[..])?;
    let mut feed_items = HashSet::new();

    for item in feed.entries() {
        let feed_item = FeedItem {
            feed_id,
            link: format!("{feed_base_url}{}", item.id().to_string()),
        };

        feed_items.insert(feed_item);
    }

    Ok(feed_items)
}

fn get_rss_feed_items(feed_id: u64, feed: Bytes) -> Result<HashSet<FeedItem>, Box<dyn Error>> {
    let feed = rss::Channel::read_from(&feed[..])?;
    let mut feed_items = HashSet::new();

    for item in feed.items() {
        let feed_item = FeedItem {
            feed_id,
            link: item.link().ok_or(FeedError::FeedItemReadError)?.to_string(),
        };

        feed_items.insert(feed_item);
    }

    println!("feed_items: {feed_items:?}");
    Ok(feed_items)
}

#[derive(Debug, thiserror::Error)]
pub enum FeedError {
    #[allow(dead_code)]
    #[error("Unable to read al lfields from the feed_item")]
    FeedItemReadError,
}


#[cfg(test)]
mod test {
    #[test]
    fn test_atom_can_get_feed_title() {
        use super::get_atom_feed_items;
        let feed = reqwest::blocking::get("https://ragnyll.github.io/feed.xml")
            .unwrap()
            .bytes()
            .unwrap();

        let feed_items = get_atom_feed_items("https://ragnyll.github.io", 64, feed);
        println!("feed_items {feed_items:?}");
        assert_eq!(true, false);
    }
}
