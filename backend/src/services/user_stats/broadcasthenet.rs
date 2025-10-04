use std::time::Duration;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    models::{
        indexer::{Indexer, Scraper},
        user_stats::UserProfileScraped,
    },
};

pub struct BroadcasthenetScraper;

#[derive(Debug, Deserialize)]
struct BroadcasthenetResponse {
    result: Option<UserProfileScrapedContent>,
    error: Option<BTNError>,
}

#[derive(Debug, Deserialize)]
struct BTNError {
    message: String,
}

#[derive(Debug, Deserialize)]
struct UserProfileScrapedContent {
    #[serde(rename = "Upload")]
    uploaded: String,
    #[serde(rename = "Download")]
    downloaded: String,
    #[serde(rename = "Bonus")]
    bonus_points: String,
    #[serde(rename = "Class")]
    class: String,
    #[serde(rename = "Snatches")]
    snatched: String,
}

impl From<UserProfileScrapedContent> for UserProfileScraped {
    fn from(wrapper: UserProfileScrapedContent) -> Self {
        UserProfileScraped {
            uploaded: wrapper.uploaded.parse().unwrap_or(0),
            downloaded: wrapper.downloaded.parse().unwrap_or(0),
            bonus_points: Some(
                wrapper
                    .bonus_points
                    .split_once('.')
                    .unwrap()
                    .0
                    .parse()
                    .unwrap_or(0),
            ),
            class: wrapper.class,
            snatched: Some(wrapper.snatched.parse().unwrap_or(0)),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize)]
struct BroadcasthenetRequestBody {
    method: String,
    params: Vec<String>,
    jsonrpc: String,
    id: u8,
}

#[async_trait]
impl Scraper for BroadcasthenetScraper {
    async fn scrape(
        &self,
        indexer: Indexer,
        client: &reqwest::Client,
    ) -> Result<UserProfileScraped> {
        let res = client
            .post("https://api.broadcasthe.net/")
            .json(&BroadcasthenetRequestBody {
                params: vec![
                    indexer
                        .auth_data
                        .get("api_key")
                        .ok_or("btn API key not found")
                        .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
                        .get("value")
                        .ok_or("btn API key value not found")
                        .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
                        .as_str()
                        .unwrap()
                        .to_string(),
                ],
                id: 1,
                jsonrpc: "2.0".to_string(),
                method: "userInfo".to_string(),
            })
            // btn sometimes has very long response times
            .timeout(Duration::from_secs(120))
            .send()
            .await
            .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?;

        let body = res.text().await.unwrap();
        let response = serde_json::from_str::<BroadcasthenetResponse>(&body)
            .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?;
        if response.result.is_none() {
            return Err(Error::CouldNotScrapeIndexer(
                response.error.unwrap_or(BTNError { message: body }).message,
            ));
        }

        Ok(response.result.unwrap().into())
    }
}
