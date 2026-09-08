use async_trait::async_trait;
use serde::Deserialize;

use crate::{
    error::{Error, Result},
    models::{
        indexer::{Indexer, Scraper},
        user_stats::UserProfileScraped,
    },
};

#[derive(Debug, Deserialize)]
struct UploadCXResponse {
    group: String,
    uploaded: i64,
    downloaded: i64,
    ratio: f32,
    seeding: i32,
    leeching: i32,
    seedbonus: String,
}

impl From<UploadCXResponse> for UserProfileScraped {
    fn from(response: UploadCXResponse) -> Self {
        UserProfileScraped {
            uploaded: response.uploaded,
            downloaded: response.downloaded,
            seeding: Some(response.seeding),
            leeching: Some(response.leeching),
            class: response.group,
            bonus_points: Some(response.seedbonus.parse::<f64>().unwrap_or(0.0).round() as i64),
            ratio: response.ratio,
            ..Default::default()
        }
    }
}

pub struct UploadCXScraper;

#[async_trait]
impl Scraper for UploadCXScraper {
    async fn scrape(
        &self,
        indexer: Indexer,
        client: &reqwest::Client,
    ) -> Result<UserProfileScraped> {
        let api_key = indexer
            .auth_data
            .get("api_key")
            .ok_or("UploadCX API key not found")
            .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
            .get("value")
            .ok_or("UploadCX API key value not found")
            .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
            .as_str()
            .unwrap();

        let res = client
            .get("https://upload.cx/api/user")
            .bearer_auth(api_key)
            .send()
            .await
            .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?;

        let body = res
            .text()
            .await
            .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?;

        let response = serde_json::from_str::<UploadCXResponse>(&body).map_err(|e| {
            Error::CouldNotScrapeIndexer(format!("Your api key is probably invalid. {}", e))
        })?;

        Ok(response.into())
    }
}
