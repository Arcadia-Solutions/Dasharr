use async_trait::async_trait;
use serde::Deserialize;

use crate::{
    error::{Error, Result},
    models::{
        indexer::{Indexer, Scraper},
        user_stats::UserProfileScraped,
    },
};

pub struct PhoenixProjectScraper;

//------------- For action=user
#[derive(Debug, Deserialize)]
struct PhoenixProjectProfileResponse {
    response: Option<UserProfileScrapedContent>,
    status: String,
    error: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsonStats {
    // last_access: NaiveDateTime,
    uploaded: i64,
    downloaded: i64,
    ratio: f32,
    required_ratio: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsonRanks {
    #[serde(rename = "uploaded")]
    rank_uploaded: i32,
    #[serde(rename = "downloaded")]
    rank_downloaded: i32,
    uploads: i32,
    requests: i32,
    bounty: i32,
    posts: i32,
    overall: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsonPersonal {
    class: String,
    // paranoia: i32,
    paranoia_text: String,
    donor: bool,
    warned: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsonCommunity {
    posts: i32,
    torrent_comments: i32,
    collages_started: i32,
    collages_contrib: i32,
    requests_filled: i32,
    requests_voted: i32,
    #[serde(rename = "uploaded")]
    uploaded_torrents: i32,
    groups: i32,
    seeding: i32,
    leeching: i32,
    snatched: i32,
    invited: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserProfileScrapedContent {
    avatar: String,
    stats: JsonStats,
    ranks: JsonRanks,
    personal: JsonPersonal,
    community: JsonCommunity,
}
//------------- For action=user

//------------- For action=index
#[derive(Debug, Deserialize)]
pub struct PhoenixProjectIndexResponse {
    pub status: String,
    pub error: Option<String>,
    pub response: Option<Index>,
}

#[derive(Debug, Deserialize)]
pub struct Index {
    // pub username: String,
    // pub id: i64,
    // pub authkey: String,
    // pub passkey: String,
    // pub notifications: Notifications,
    pub userstats: UserStats,
}

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Notifications {
//     pub messages: i64,
//     pub notifications: i64,
//     pub new_announcement: bool,
//     pub new_blog: bool,
// }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStats {
    // pub uploaded: i64,
    // pub downloaded: i64,
    // pub ratio: f32,
    // pub required_ratio: f32,
    pub bonus_points: i64,
    pub bonus_points_per_hour: f32,
    // pub class: String,
}
//------------- For action=index

impl From<UserProfileScrapedContent> for UserProfileScraped {
    fn from(wrapper: UserProfileScrapedContent) -> Self {
        UserProfileScraped {
            avatar: wrapper.avatar,
            // last_access: wrapper.stats.last_access,
            uploaded: wrapper.stats.uploaded,
            downloaded: wrapper.stats.downloaded,
            ratio: wrapper.stats.ratio,
            required_ratio: Some(wrapper.stats.required_ratio),
            rank_uploaded: Some(wrapper.ranks.rank_uploaded),
            rank_downloaded: Some(wrapper.ranks.rank_downloaded),
            rank_uploads: Some(wrapper.ranks.uploads),
            rank_requests: Some(wrapper.ranks.requests),
            rank_bounty: Some(wrapper.ranks.bounty),
            rank_posts: Some(wrapper.ranks.posts),
            rank_artists: None,
            rank_overall: Some(wrapper.ranks.overall),
            class: wrapper.personal.class,
            // paranoia: Some(wrapper.personal.paranoia),
            paranoia_text: Some(wrapper.personal.paranoia_text),
            donor: Some(wrapper.personal.donor),
            warned: Some(wrapper.personal.warned),
            posts: Some(wrapper.community.posts),
            torrent_comments: Some(wrapper.community.torrent_comments),
            collages_started: Some(wrapper.community.collages_started),
            collages_contrib: Some(wrapper.community.collages_contrib),
            requests_filled: Some(wrapper.community.requests_filled),
            requests_voted: Some(wrapper.community.requests_voted),
            uploaded_torrents: Some(wrapper.community.uploaded_torrents),
            groups: Some(wrapper.community.groups),
            seeding: Some(wrapper.community.seeding),
            leeching: Some(wrapper.community.leeching),
            snatched: Some(wrapper.community.snatched),
            invited: Some(wrapper.community.invited),
            bonus_points: None,
            bonus_points_per_hour: None,
        }
    }
}

#[async_trait]
impl Scraper for PhoenixProjectScraper {
    async fn scrape(&self, indexer: Indexer) -> Result<UserProfileScraped> {
        let client = reqwest::Client::new();
        let user_id = indexer
            .auth_data
            .get("user_id")
            .ok_or("phoenix project user_id not found")
            .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
            .get("value")
            .ok_or("phoenix project user_id value not found")
            .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
            .as_str()
            .unwrap();
        let api_key = indexer
            .auth_data
            .get("api_key")
            .ok_or("phoenix project API key not found")
            .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
            .get("value")
            .ok_or("phoenix project API key value not found")
            .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
            .as_str()
            .unwrap();
        let res = client
            .get(format!(
                "https://phoenixproject.app/ajax.php?action=user&id={}",
                user_id
            ))
            .header("Authorization", api_key)
            .send()
            .await
            .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?;
        let body = res.text().await.unwrap();

        let response = serde_json::from_str::<PhoenixProjectProfileResponse>(&body)
            .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?;
        if response.status != "success" {
            return Err(Error::CouldNotScrapeIndexer(
                response.error.unwrap_or(response.status),
            ));
        }
        let mut profile: UserProfileScraped = response.response.unwrap().into();

        // bonus points are only available on another endpoint
        let res = client
            .get("https://phoenixproject.app/ajax.php?action=index")
            .header("Authorization", api_key)
            .send()
            .await
            .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?;
        let body = res.text().await.unwrap();
        let response = serde_json::from_str::<PhoenixProjectIndexResponse>(&body)
            .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?;
        if response.status != "success" {
            return Err(Error::CouldNotScrapeIndexer(
                response.error.unwrap_or(response.status),
            ));
        }
        let index = response.response.unwrap();
        profile.bonus_points = Some(index.userstats.bonus_points);
        profile.bonus_points_per_hour = Some(index.userstats.bonus_points_per_hour);

        Ok(profile)
    }
}
