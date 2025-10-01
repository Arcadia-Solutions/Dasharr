use async_trait::async_trait;
use serde::Deserialize;

use crate::{
    error::{Error, Result},
    models::{
        indexer::{Indexer, Scraper},
        user_stats::UserProfileScraped,
    },
};

pub struct GazelleGamesScraper;

#[derive(Debug, Deserialize)]
struct GazelleGamesResponse {
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
    ratio: String,
    required_ratio: Option<String>,
    #[serde(rename = "gold")]
    bonus_points: Option<i64>,
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
    posts: Option<i32>,
    torrent_comments: Option<i32>,
    collections: Option<i32>,
    // collages_contrib: None,
    requests_filled: Option<i32>,
    requests_voted: Option<i32>,
    #[serde(rename = "uploaded")]
    uploaded_torrents: Option<i32>,
    groups: Option<i32>,
    seeding: Option<i32>,
    leeching: Option<i32>,
    snatched: Option<i32>,
    invited: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserProfileScrapedContent {
    avatar: String,
    stats: JsonStats,
    personal: JsonPersonal,
    community: JsonCommunity,
}

impl From<UserProfileScrapedContent> for UserProfileScraped {
    fn from(wrapper: UserProfileScrapedContent) -> Self {
        UserProfileScraped {
            avatar: wrapper.avatar,
            // last_access: wrapper.stats.last_access,
            uploaded: wrapper.stats.uploaded,
            downloaded: wrapper.stats.downloaded,
            ratio: wrapper.stats.ratio.parse().unwrap_or(0.0),
            required_ratio: Some(
                wrapper
                    .stats
                    .required_ratio
                    .unwrap_or("0.0".to_string())
                    .parse()
                    .unwrap_or(0.0),
            ),
            class: wrapper.personal.class,
            // paranoia: Some(wrapper.personal.paranoia),
            paranoia_text: Some(wrapper.personal.paranoia_text),
            donor: Some(wrapper.personal.donor),
            warned: Some(wrapper.personal.warned),
            posts: Some(wrapper.community.posts.unwrap_or(0)),
            torrent_comments: Some(wrapper.community.torrent_comments.unwrap_or(0)),
            collages_started: Some(wrapper.community.collections.unwrap_or(0)),
            requests_filled: Some(wrapper.community.requests_filled.unwrap_or(0)),
            requests_voted: Some(wrapper.community.requests_voted.unwrap_or(0)),
            uploaded_torrents: Some(wrapper.community.uploaded_torrents.unwrap_or(0)),
            groups: Some(wrapper.community.groups.unwrap_or(0)),
            seeding: Some(wrapper.community.seeding.unwrap_or(0)),
            leeching: Some(wrapper.community.leeching.unwrap_or(0)),
            snatched: Some(wrapper.community.snatched.unwrap_or(0)),
            invited: Some(wrapper.community.invited.unwrap_or(0)),
            bonus_points: Some(wrapper.stats.bonus_points.unwrap_or(0)),
            rank_artists: None,
            rank_bounty: None,
            rank_downloaded: None,
            rank_overall: None,
            rank_posts: None,
            rank_requests: None,
            rank_uploaded: None,
            rank_uploads: None,
            collages_contrib: None,
        }
    }
}

#[async_trait]
impl Scraper for GazelleGamesScraper {
    async fn scrape(&self, indexer: Indexer) -> Result<UserProfileScraped> {
        let client = reqwest::Client::new();
        let res = client
            .get(format!(
                "https://gazellegames.net/api.php?request=user&id={}",
                indexer
                    .auth_data
                    .get("user_id")
                    .ok_or("ggn user_id not found")
                    .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
                    .get("value")
                    .ok_or("ggn user_id value not found")
                    .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
                    .as_str()
                    .unwrap()
            ))
            .header(
                "X-API-Key",
                indexer
                    .auth_data
                    .get("api_key")
                    .ok_or("ggn API key not found")
                    .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?
                    .get("value")
                    .ok_or("ggn API key value not found")
                    .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?
                    .as_str()
                    .unwrap(),
            )
            .send()
            .await
            .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?;

        let body = res.text().await.unwrap();
        let response = serde_json::from_str::<GazelleGamesResponse>(&body)
            .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?;

        if response.status != "success" {
            return Err(Error::CouldNotScrapeIndexer(
                response.error.unwrap_or(response.status),
            ));
        }

        Ok(response.response.unwrap().into())
    }
}
