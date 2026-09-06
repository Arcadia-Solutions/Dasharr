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
#[allow(dead_code)]
struct BlutopiaUserData {
    username: String,
    group: String,
    uploaded: i64,
    downloaded: i64,
    ratio: f64,
    buffer: i64,
    seeding: i32,
    leeching: i32,
    seedbonus: String,
    hit_and_runs: i32,
}

pub struct BlutopiaScraper;

#[async_trait]
impl Scraper for BlutopiaScraper {
    async fn scrape(
        &self,
        indexer: Indexer,
        client: &reqwest::Client,
    ) -> Result<UserProfileScraped> {
        let api_key = indexer
            .auth_data
            .get("api_key")
            .ok_or("BLU API key not found")
            .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
            .get("value")
            .ok_or("BLU API key value not found")
            .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
            .as_str()
            .unwrap();

        let res = client
            .get("https://blutopia.cc/api/user")
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
        let data: BlutopiaUserData = serde_json::from_str(&body).map_err(|e| {
            Error::CouldNotScrapeIndexer(format!("Your api key is probably invalid. {}", e))
        })?;

        Ok(UserProfileScraped {
            uploaded: data.uploaded,
            downloaded: data.downloaded,
            ratio: data.ratio as f32,
            class: data.group,
            seeding: Some(data.seeding),
            leeching: Some(data.leeching),
            bonus_points: Some(data.seedbonus.parse::<f64>().unwrap_or(0.0).round() as i64),
            ..Default::default()
        })
    }
}
