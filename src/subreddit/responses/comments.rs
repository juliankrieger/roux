use crate::responses::BasicListing;
use serde::Deserialize;

// Everything is an option to deal with both `latest_comments` and `article_comments`
#[derive(Debug, Deserialize)]
pub struct CommentsData {
    pub total_awards_received: Option<i32>,
    pub approved_at_utc: Option<f64>,
    // TODO: `edited` is sometimes a bool and somtimes i32 :/
    // pub edited: i32,
    pub link_id: Option<String>,
    pub author_flair_template_id: Option<String>,
    pub likes: Option<bool>,
    pub saved: Option<bool>,
    pub id: Option<String>,
    pub gilded: Option<i32>,
    pub archived: Option<bool>,
    pub no_follow: Option<bool>,
    pub author: Option<String>,
    pub can_mod_post: Option<bool>,
    pub created_utc: Option<f64>,
    pub send_replies: Option<bool>,
    pub parent_id: Option<String>,
    pub score: Option<i32>,
    pub author_fullname: Option<String>,
    pub over_18: Option<bool>,
    pub approved_by: Option<String>,
    pub subreddit_id: Option<String>,
    pub body: Option<String>,
    pub link_title: Option<String>,
    pub name: Option<String>,
    pub author_patreon_flair: Option<bool>,
    pub downs: Option<i32>,
    pub is_submitter: Option<bool>,
    pub body_html: Option<String>,
    pub distinguished: Option<String>,
    pub stickied: Option<bool>,
    pub author_premium: Option<bool>,
    pub can_gild: Option<bool>,
    pub subreddit: Option<String>,
    pub author_flair_text_color: Option<String>,
    pub score_hidden: Option<bool>,
    pub permalink: Option<String>,
    pub num_reports: Option<i32>,
    pub link_permalink: Option<String>,
    pub link_author: Option<String>,
    pub subreddit_name_prefixed: Option<String>,
    pub author_flair_text: Option<String>,
    pub link_url: Option<String>,
    pub created: Option<f64>,
    pub collapsed: Option<bool>,
    pub controversiality: Option<i32>,
    pub locked: Option<bool>,
    pub quarantine: Option<bool>,
    pub subreddit_type: Option<String>,
    pub ups: Option<i32>,
}

pub type Comments = BasicListing<CommentsData>;
