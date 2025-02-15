//! # Subreddit Comment Responses
use serde::Deserialize;

use crate::models::response::BasicListing;
use crate::models::comment::CommentData;

/// Doc
pub type Replies = BasicListing<Box<CommentData>>;

/// Replies can be more comments or an empty string
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MaybeReplies {
    /// Reply
    Reply(Replies),
    /// String
    Str(String),
}