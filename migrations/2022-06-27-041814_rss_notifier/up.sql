CREATE TABLE  channel (
    -- Individual discord channel ids
    channel_id INTEGER NOT NULL PRIMARY KEY,
    -- The name of the channel
    channel_name VARCHAR,
    -- SQLITE dosent support arrays, so this represents feed_ids on the rss_feed_url table
    -- ex: feed_subscriptions: 1,2,3
    -- may corresponds to the rss_feed_url(s) with the PRIMARY_KEY id 1, 2, and 3
    -- this leaves it up to the client to deserialize this
    -- Hopefully this wont be the case when i move to a better database
    feed_subscriptions VARCHAR
)
