use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use soa_derive::StructOfArray;
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Default, Deserialize, Serialize, FromRow, StructOfArray, ToSchema)]
#[soa_attr(Vec, derive(Deserialize, Serialize, ToSchema))]
pub struct UserProfileScraped {
    pub avatar: String,
    // #[schema(value_type = String, format = DateTime)]
    // pub last_access: NaiveDateTime,
    pub uploaded: i64,
    pub downloaded: i64,
    pub ratio: f32,
    pub required_ratio: Option<f32>,
    pub rank_uploaded: Option<i32>,
    pub rank_downloaded: Option<i32>,
    pub rank_uploads: Option<i32>,
    pub rank_requests: Option<i32>,
    pub rank_bounty: Option<i32>,
    pub rank_posts: Option<i32>,
    pub rank_artists: Option<i32>,
    pub rank_overall: Option<f32>,
    pub class: String,
    // pub paranoia: Option<i32>,
    pub paranoia_text: Option<String>,
    pub donor: Option<bool>,
    pub warned: Option<bool>,
    pub posts: Option<i32>,
    pub torrent_comments: Option<i32>,
    pub collages_started: Option<i32>,
    pub collages_contrib: Option<i32>,
    pub requests_filled: Option<i32>,
    pub requests_voted: Option<i32>,
    pub uploaded_torrents: Option<i32>,
    pub groups: Option<i32>,
    pub seeding: Option<i32>,
    pub leeching: Option<i32>,
    pub snatched: Option<i32>,
    pub invited: Option<i32>,
    pub bonus_points: Option<i64>,
    pub bonus_points_per_hour: Option<f32>,
}

#[derive(Debug, Default, Deserialize, Serialize, FromRow, StructOfArray, ToSchema)]
#[soa_attr(Vec, derive(Deserialize, Serialize, ToSchema))]
pub struct UserProfile {
    #[sqlx(flatten)]
    #[serde(flatten)]
    #[nested_soa]
    pub profile: UserProfileScraped,
    #[schema(value_type = String, format = DateTime)]
    pub scraped_at: DateTime<Local>,
    pub indexer_id: i32,
}

impl UserProfileVec {
    pub fn from_vec(profiles: Vec<UserProfile>) -> Self {
        profiles.into_iter().collect()
    }
}
