//! A mock of a database of posts.

use std::time::Duration;

use itertools::Itertools;
use phf::{Map, phf_map};
use crate::model::Post;

/// Latency of DB operation.
const DB_LATENCY: Duration = Duration::from_millis(250);

/// Representation of post in the database.
struct DbPost {
    title: &'static str,
    body: &'static str,
}

impl From<&DbPost> for Post {
    fn from(value: &DbPost) -> Self {
        Self {
            title: value.title.to_string(),
            body: value.body.to_string(),
        }
    }
}

/// A map of post entries in the database.
static ENTRIES: Map<u64, DbPost> = phf_map! {
    1u64 => DbPost { title: "First post", body: "This is the very first post." },
    2u64 => DbPost { title: "Second post", body: "This is yet another post.\nIt even has another line." },
    42u64 => DbPost { title: "Last post", body: "This is yet another post.\nBut also the last one.\nFor sure!\n" },
};

/// Fetch all posts from the database.
pub async fn all_posts() -> impl Iterator<Item = (u64, Post)> {
    tokio::time::sleep(DB_LATENCY).await;
    ENTRIES.into_iter().sorted_by_key(|(id, _post)| **id).map(|(id, post)| (*id, Post::from(post)))
}

/// Fetch a post by ID from the database.
#[must_use]
pub async fn post_by_id(id: u64) -> Option<Post> {
    tokio::time::sleep(DB_LATENCY).await;
    ENTRIES.get(&id).map(Post::from)
}