/// The feeds module is in charge of determining the feeds type (RSS or Atom) deserializing it into an object and providing bindings for inserting it into a db.

/// Not all feeds are RSS. What people call RSS actually usually includes atom feeds, which are far
/// more common these these days. Both are valid and need to be handled.
use std::error::Error;
async fn example_feed() -> Result<atom_syndication::Feed, Box<dyn Error>> {
    let content = reqwest::get("https://ragnyll.github.io/feed.xml")
        .await?
        .bytes()
        .await?;
    let feed = atom_syndication::Feed::read_from(&content[..]).unwrap();
    println!("{:?}", feed.base());

    Ok(feed)
}
