use async_trait::async_trait;
use serde::Deserialize;

use crate::{
    error::{Error, Result},
    models::{
        indexer::{Indexer, Scraper},
        user_stats::UserProfileScraped,
    },
    services::user_stats::common::unit3d::UnitedResponse,
};

pub struct LSTScraper;

#[derive(Debug, Deserialize)]
struct LSTResponse {
    data: UnitedResponse,
}

impl From<LSTResponse> for UserProfileScraped {
    fn from(response: LSTResponse) -> Self {
        response.data.into()
    }
}

#[async_trait]
impl Scraper for LSTScraper {
    async fn scrape(
        &self,
        indexer: Indexer,
        client: &reqwest::Client,
    ) -> Result<UserProfileScraped> {
        let api_key = indexer
            .auth_data
            .get("api_key")
            .ok_or("LST API key not found")
            .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
            .get("value")
            .ok_or("LST API key value not found")
            .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
            .as_str()
            .unwrap();

        let res = client
            .get("https://lst.gg/api/user")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?;

        let body = res
            .text()
            .await
            .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?;
        let response = serde_json::from_str::<LSTResponse>(&body).map_err(|e| {
            Error::CouldNotScrapeIndexer(format!("Failed to parse LST response: {}", e))
        })?;

        Ok(response.into())
    }
}
