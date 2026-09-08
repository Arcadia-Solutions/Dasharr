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
struct AitherUserData {
    username: String,
    group: String,
    uploaded: u64,
    downloaded: u64,
    ratio: String,
    buffer: i64,
    seeding: i32,
    leeching: i32,
    seedbonus: String,
    hit_and_runs: i32,
    active_hit_and_runs: i32,
    seeding_size: i64,
    avg_seedtime: i64,
    total_seedtime: i64,
    total_uploads: i32,
    torrent_uploads_month: i32,
    unread_pms: i32,
    unread_notifications: i32,
    is_donor: bool,
}

pub struct AitherScraper;

#[async_trait]
impl Scraper for AitherScraper {
    async fn scrape(
        &self,
        indexer: Indexer,
        client: &reqwest::Client,
    ) -> Result<UserProfileScraped> {
        let api_key = indexer
            .auth_data
            .get("api_key")
            .ok_or("ATH API key not found")
            .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
            .get("value")
            .ok_or("ATH API key value not found")
            .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
            .as_str()
            .unwrap();

        let res = client
            .get("https://aither.cc/api/user")
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
        let data: AitherUserData = serde_json::from_str(&body).map_err(|e| {
            Error::CouldNotScrapeIndexer(format!("Your api key is probably invalid. {}", e))
        })?;

        Ok(UserProfileScraped {
            uploaded: data.uploaded as i64,
            downloaded: data.downloaded as i64,
            ratio: data.ratio.parse::<f32>().unwrap_or(0.0),
            class: data.group,
            seeding: Some(data.seeding),
            leeching: Some(data.leeching),
            bonus_points: Some(data.seedbonus.parse::<f64>().unwrap_or(0.0).round() as i64),
            donor: Some(data.is_donor),
            seed_size: Some(data.seeding_size),
            average_seed_time: Some(data.avg_seedtime),
            uploaded_torrents: Some(data.total_uploads),
            ..Default::default()
        })
    }
}
