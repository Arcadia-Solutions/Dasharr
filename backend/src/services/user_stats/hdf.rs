use async_trait::async_trait;
use serde::Deserialize;

use crate::{
    error::{Error, Result},
    models::{
        indexer::{Indexer, Scraper},
        user_stats::UserProfileScraped,
    },
};

pub struct HDFScraper;

#[derive(Debug, Deserialize)]
struct HDFResponse {
    response: HDFUser,
}

#[derive(Debug, Deserialize)]
struct HDFUser {
    class: String,
    uploaded: i64,
    downloaded: i64,
    ratio: f32,
    required_ratio: f32,
    bonus_points: i64,
}

impl From<HDFUser> for UserProfileScraped {
    fn from(user: HDFUser) -> Self {
        UserProfileScraped {
            uploaded: user.uploaded,
            downloaded: user.downloaded,
            ratio: user.ratio,
            required_ratio: Some(user.required_ratio),
            class: user.class,
            bonus_points: Some(user.bonus_points),
            ..Default::default()
        }
    }
}

#[async_trait]
impl Scraper for HDFScraper {
    async fn scrape(
        &self,
        indexer: Indexer,
        client: &reqwest::Client,
    ) -> Result<UserProfileScraped> {
        let res = client
            .get("https://hdf.world/api.php?endpoint=self")
            .header(
                "X-Api-Key",
                indexer
                    .auth_data
                    .get("api_key")
                    .ok_or("HDF API key not found")
                    .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
                    .get("value")
                    .ok_or("HDF API key value not found")
                    .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
                    .as_str()
                    .unwrap(),
            )
            .send()
            .await
            .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?;

        let body = res
            .text()
            .await
            .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?;
        let response = serde_json::from_str::<HDFResponse>(&body).map_err(|e| {
            Error::CouldNotScrapeIndexer(format!("Your api key is probably invalid. {}", e))
        })?;

        Ok(response.response.into())
    }
}
