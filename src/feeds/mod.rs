/// The feeds module is in charge of determining the feeds type (RSS or Atom) deserializing it into an object and providing bindings for inserting it into a db.

/// Not all feeds are RSS. What people call RSS actually usually includes atom feeds, which are far
/// more common these these days. Both are valid and need to be handled.
use bytes::Bytes;
use crate::db::{QueryManager, QueryError};
use crate::models::{FeedItem, FeedSubscription, FeedType};
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
    match feed {
        Ok(f) => match subscription.feed_type {
            // FeedType::Rss => update_rss_feed_items(f, query_manager)?,
            FeedType::Atom => update_atom_feed_items(f, query_manager)?,
            _ => todo!("this shouldnt be here")
        },
        Err(e) => eprintln!(
            "I was unable to retrieve feed updates for {:?}: {:?}",
            subscription.feed_url, e
        ),
    }

    Ok(())
}

fn update_atom_feed_items(
    feed: Bytes,
    query_manager: &QueryManager,
) -> Result<(), Box<dyn Error>> {
    let feed= atom::Feed::read_from(&feed[..]);

    for item in feed {
        println!("Feed Item title: {:?}", item.title());
    }
    Ok(())
}

// fn update_rss_feed_items(
    // feed: Bytes,
    // query_manager: &QueryManager,
// ) -> Result<(), Box<dyn Error>> {
    // let feed =
    // Ok(())
// }


#[cfg(test)]
mod test {
    #[test]
    fn test_can_get_feed_title() {
        use super::update_feed_items_for_subscription;
    }
}
