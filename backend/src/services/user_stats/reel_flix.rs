use async_trait::async_trait;

use crate::{
    error::{Error, Result},
    models::{
        indexer::{Indexer, Scraper},
        user_stats::UserProfileScraped,
    },
    services::user_stats::common::unit3d::UnitedResponse,
};

pub struct ReelFlixScraper;

#[async_trait]
impl Scraper for ReelFlixScraper {
    async fn scrape(&self, indexer: Indexer) -> Result<UserProfileScraped> {
        let client = reqwest::Client::new();
        let res = client
            .get(format!(
                "https://reelflix.xyz/api/user?api_token={}",
                indexer
                    .auth_data
                    .get("api_key")
                    .ok_or("ReelFlix API key not found")
                    .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
                    .get("value")
                    .ok_or("ReelFlix API key value not found")
                    .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
                    .as_str()
                    .unwrap()
            ))
            .send()
            .await
            .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?;

        let body = res.text().await.unwrap();
        let response = serde_json::from_str::<UnitedResponse>(&body).map_err(|e| {
            Error::CouldNotScrapeIndexer(format!("Your api key is probably invalid. {}", e))
        })?;

        Ok(response.into())
    }
}
