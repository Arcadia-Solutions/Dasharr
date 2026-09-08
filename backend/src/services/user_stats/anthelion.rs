use async_trait::async_trait;
use serde::Deserialize;
use serde_json::Value;

use crate::{
    error::{Error, Result},
    models::{
        indexer::{Indexer, Scraper},
        user_stats::UserProfileScraped,
    },
};

pub struct AnthelionScraper;

#[derive(Debug, Deserialize)]
struct AnthelionResponse {
    response: Option<UserProfileScrapedContent>,
    error: Option<String>,
    status: String,
}

#[derive(Debug, Deserialize)]
struct UserProfileScrapedContent {
    #[serde(rename = "Uploaded")]
    uploaded: Value,
    #[serde(rename = "Downloaded")]
    downloaded: Value,
    #[serde(rename = "Class")]
    class: String,
    #[serde(rename = "Snatched")]
    snatched: Value,
    #[serde(rename = "ForumPosts")]
    forum_posts: Value,
    #[serde(rename = "SeedCount")]
    seeding: Value,
    #[serde(rename = "Orbs")]
    bonus_points: Value,
    #[serde(rename = "Uploads")]
    uploaded_torrents: Value,
    #[serde(rename = "SeedSize")]
    seed_size: Value,
    #[serde(rename = "Invites")]
    invited: Value,
}

fn parse_i64(value: Value) -> i64 {
    match value {
        Value::String(s) => s.parse().unwrap_or(0),
        Value::Number(n) => n.as_i64().unwrap_or(0),
        _ => 0,
    }
}

fn parse_i32(value: Value) -> Option<i32> {
    match value {
        Value::String(s) => s.parse().ok(),
        Value::Number(n) => n.as_i64().and_then(|v| i32::try_from(v).ok()),
        _ => None,
    }
}

impl From<UserProfileScrapedContent> for UserProfileScraped {
    fn from(wrapper: UserProfileScrapedContent) -> Self {
        UserProfileScraped {
            uploaded: parse_i64(wrapper.uploaded),
            downloaded: parse_i64(wrapper.downloaded),
            seeding: parse_i32(wrapper.seeding),
            class: wrapper.class,
            snatched: parse_i32(wrapper.snatched),
            posts: parse_i32(wrapper.forum_posts),
            bonus_points: parse_i32(wrapper.bonus_points).map(i64::from),
            uploaded_torrents: parse_i32(wrapper.uploaded_torrents),
            seed_size: parse_i32(wrapper.seed_size).map(i64::from),
            invited: parse_i32(wrapper.invited),
            ..Default::default()
        }
    }
}

#[async_trait]
impl Scraper for AnthelionScraper {
    async fn scrape(
        &self,
        indexer: Indexer,
        client: &reqwest::Client,
    ) -> Result<UserProfileScraped> {
        let res = client
            .post(format!(
                "https://anthelion.me/api.php?action=user&method=getuserinfo&apikey={}&type=id&user={}",
                indexer
                    .auth_data
                    .get("api_key")
                    .ok_or("ANT API key not found")
                    .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
                    .get("value")
                    .ok_or("ANT API key value not found")
                    .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
                    .as_str()
                    .unwrap(),
                indexer
                    .auth_data
                    .get("user_id")
                    .ok_or("ANT user id not found")
                    .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
                    .get("value")
                    .ok_or("ANT user id value not found")
                    .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
                    .as_str()
                    .unwrap()
            ))
            .send()
            .await
            .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?;

        let body = res
            .text()
            .await
            .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?;
        let response = serde_json::from_str::<AnthelionResponse>(&body)
            .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?;
        if response.response.is_none() {
            return Err(Error::CouldNotScrapeIndexer(
                response.error.unwrap_or(response.status),
            ));
        }

        Ok(response.response.unwrap().into())
    }
}
