use async_trait::async_trait;
use serde::Deserialize;

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
    uploaded: i64,
    #[serde(rename = "Downloaded")]
    downloaded: i64,
    #[serde(rename = "Class")]
    class: String,
    #[serde(rename = "Snatched")]
    snatched: i32,
    #[serde(rename = "ForumPosts")]
    forum_posts: i32,
    #[serde(rename = "SeedCount")]
    seeding: i32,
}

impl From<UserProfileScrapedContent> for UserProfileScraped {
    fn from(wrapper: UserProfileScrapedContent) -> Self {
        UserProfileScraped {
            uploaded: wrapper.uploaded,
            downloaded: wrapper.downloaded,
            seeding: Some(wrapper.seeding),
            class: wrapper.class,
            snatched: Some(wrapper.snatched),
            posts: Some(wrapper.forum_posts),
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

        let body = res.text().await.unwrap();
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
