use async_trait::async_trait;
use serde::Deserialize;

use crate::{
    error::{Error, Result},
    models::{
        indexer::{Indexer, Scraper},
        user_stats::UserProfileScraped,
    },
    services::helpers::bytes_from_size_string,
};

pub struct LSTScraper;

#[derive(Debug, Deserialize)]
struct LSTResponse {
    group: String,
    uploaded: String,
    downloaded: String,
    ratio: String,
    seeding: i32,
    leeching: i32,
    seedbonus: String,
}

impl From<LSTResponse> for UserProfileScraped {
    fn from(wrapper: LSTResponse) -> Self {
        UserProfileScraped {
            uploaded: bytes_from_size_string(&wrapper.uploaded),
            downloaded: bytes_from_size_string(&wrapper.downloaded),
            seeding: Some(wrapper.seeding),
            leeching: Some(wrapper.leeching),
            class: wrapper.group,
            bonus_points: Some(wrapper.seedbonus.parse::<f64>().unwrap_or(0.0).round() as i64),
            ratio: wrapper.ratio.parse::<f32>().unwrap_or(0.0),
            ..Default::default()
        }
    }
}

#[async_trait]
impl Scraper for LSTScraper {
    async fn scrape(&self, indexer: Indexer) -> Result<UserProfileScraped> {
        let client = reqwest::Client::new();
        let res = client
            .get(format!(
                "https://lst.gg/api/user?api_token={}",
                indexer
                    .auth_data
                    .get("api_key")
                    .ok_or("ATH API key not found")
                    .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
                    .get("value")
                    .ok_or("ATH API key value not found")
                    .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
                    .as_str()
                    .unwrap()
            ))
            .send()
            .await
            .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?;

        let body = res.text().await.unwrap();
        let response = serde_json::from_str::<LSTResponse>(&body).map_err(|e| {
            Error::CouldNotScrapeIndexer(format!("Your api key is probably invalid. {}", e))
        })?;

        Ok(response.into())
    }
}
