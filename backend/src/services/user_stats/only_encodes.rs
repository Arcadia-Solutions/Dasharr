use async_trait::async_trait;

use crate::{
    error::{Error, Result},
    models::{
        indexer::{Indexer, Scraper},
        user_stats::UserProfileScraped,
    },
    services::user_stats::common::unit3d::UnitedResponse,
};

pub struct OnlyEncodesScraper;

#[async_trait]
impl Scraper for OnlyEncodesScraper {
    async fn scrape(
        &self,
        indexer: Indexer,
        client: &reqwest::Client,
    ) -> Result<UserProfileScraped> {
        let res = client
            .get(format!(
                "https://onlyencodes.cc/api/user?api_token={}",
                indexer
                    .auth_data
                    .get("api_key")
                    .ok_or("OnlyEncodes API key not found")
                    .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
                    .get("value")
                    .ok_or("OnlyEncodes API key value not found")
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
