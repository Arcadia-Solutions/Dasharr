use chrono::{DateTime, Local};
use serde::Deserialize;
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Default, Deserialize, FromRow, ToSchema)]
pub struct UserProfileScraped {
    pub avatar: String,
    // #[schema(value_type = String, format = DateTime)]
    // pub last_access: NaiveDateTime,
    pub uploaded: u64,
    pub downloaded: u64,
    pub ratio: f32,
    pub required_ratio: f32,
    pub rank_uploaded: Option<u32>,
    pub rank_downloaded: Option<u32>,
    pub rank_uploads: Option<u32>,
    pub rank_requests: Option<u32>,
    pub rank_bounty: Option<u32>,
    pub rank_posts: Option<u32>,
    pub rank_artists: Option<u32>,
    pub rank_overall: Option<u32>,
    pub class: String,
    pub paranoia: Option<u32>,
    pub paranoia_text: Option<String>,
    pub donor: Option<bool>,
    pub warned: Option<bool>,
    pub posts: Option<u32>,
    pub torrent_comments: Option<u32>,
    pub collages_started: Option<u32>,
    pub collages_contrib: Option<u32>,
    pub requests_filled: Option<u32>,
    pub requests_voted: Option<u32>,
    pub uploaded_torrents: Option<u32>,
    pub groups: Option<u32>,
    pub seeding: Option<u32>,
    pub leeching: Option<u32>,
    pub snatched: Option<u32>,
    pub invited: Option<u32>,
}

#[derive(Debug, Default, Deserialize, FromRow, ToSchema)]
pub struct UserProfile {
    #[sqlx(flatten)]
    #[serde(flatten)]
    pub base: UserProfileScraped,
    #[schema(value_type = String, format = DateTime)]
    pub scraped_at: DateTime<Local>,
    pub indexer_id: i32,
}
