use std::time::Duration;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::models::{
    indexer::{Indexer, Scraper},
    user_stats::UserProfileScraped,
};

pub struct BroadcasthenetScraper;

#[derive(Debug, Deserialize)]
struct BroadcasthenetResponse {
    result: UserProfileScrapedContent,
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
    ) -> Result<UserProfileScraped, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let res = client
            .post("https://api.broadcasthe.net/")
            .json(&BroadcasthenetRequestBody {
                params: vec![
                    indexer
                        .auth_data
                        .get("api_key")
                        .ok_or("ggn API key not found")?
                        .get("value")
                        .ok_or("ggn API key value not found")?
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
            .await?;

        let body = res.text().await?;
        let profile: UserProfileScraped = serde_json::from_str::<BroadcasthenetResponse>(&body)?
            .result
            .into();

        Ok(profile)
    }
}
