use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use soa_derive::StructOfArray;
use sqlx::prelude::FromRow;
use std::fmt;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
#[serde(rename_all = "snake_case")]
pub enum StatsInterval {
    Day,
    Week,
    Month,
}

impl fmt::Display for StatsInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatsInterval::Day => write!(f, "day"),
            StatsInterval::Week => write!(f, "week"),
            StatsInterval::Month => write!(f, "month"),
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize, FromRow, StructOfArray, ToSchema)]
#[soa_attr(Vec, derive(Deserialize, Serialize, ToSchema))]
pub struct UserProfileScraped {
    pub avatar: String,
    // #[schema(value_type = String, format = DateTime)]
    // pub last_access: NaiveDateTime,
    pub uploaded: i64,
    pub uploaded_real: Option<i64>,
    pub downloaded: i64,
    pub downloaded_real: Option<i64>,
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
    pub seed_size: Option<i64>,
    pub average_seed_time: Option<i64>,
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

#[derive(Serialize, ToSchema)]
pub struct IndexerStats {
    pub indexer_id: i32,
    #[schema(value_type = Vec<String>, format = DateTime)]
    pub scraped_at: Vec<DateTime<Local>>,
    pub profile: UserProfileScrapedVec,
}

impl IndexerStats {
    pub fn group_by_indexer(profiles: Vec<UserProfile>) -> Vec<Self> {
        let mut groups: std::collections::BTreeMap<i32, Vec<UserProfile>> =
            std::collections::BTreeMap::new();
        for profile in profiles {
            groups.entry(profile.indexer_id).or_default().push(profile);
        }
        groups
            .into_iter()
            .map(|(indexer_id, profiles)| {
                let scraped_at = profiles.iter().map(|p| p.scraped_at).collect();
                let profile = profiles.into_iter().map(|p| p.profile).collect();
                IndexerStats {
                    indexer_id,
                    scraped_at,
                    profile,
                }
            })
            .collect()
    }
}
