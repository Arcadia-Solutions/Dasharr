use async_trait::async_trait;
use serde::Deserialize;

use crate::{
    error::{Error, Result},
    models::{
        indexer::{Indexer, Scraper},
        user_stats::UserProfileScraped,
    },
};

pub struct AnimeBytesScraper;

#[derive(Debug, Deserialize)]
struct AnimeBytesResponse {
    success: bool,
    error: Option<String>,
    yen: Option<YenStats>,
    upload: Option<UploadStats>,
    download: Option<DownloadStats>,
    torrents: Option<TorrentsStats>,
    tracker: Option<TrackerStats>,
}

#[derive(Debug, Deserialize)]
struct YenStats {
    // per_day: f32,
    per_hour: f32,
    current: i64,
}

#[derive(Debug, Deserialize)]
struct UploadStats {
    raw: i64,
    account: i64,
}

#[derive(Debug, Deserialize)]
struct DownloadStats {
    raw: i64,
    account: i64,
}

#[derive(Debug, Deserialize)]
struct TorrentsStats {
    uploaded: i32,
    // pruned: i64,
}

#[derive(Debug, Deserialize)]
struct TrackerStats {
    seeding: i32,
    leeching: i32,
    snatched: i32,
    seed_size: i64,
    avg_seed_time: i64,
}

impl From<AnimeBytesResponse> for UserProfileScraped {
    fn from(wrapper: AnimeBytesResponse) -> Self {
        let AnimeBytesResponse {
            yen: Some(yen_stats),
            upload: Some(upload_stats),
            download: Some(download_stats),
            torrents: Some(torrents_stats),
            tracker: Some(tracker_stats),
            ..
        } = wrapper
        else {
            panic!("Unexpected None value in AnimeBytesResponse.")
        };
        UserProfileScraped {
            uploaded: upload_stats.account,
            uploaded_real: Some(upload_stats.raw),
            downloaded: download_stats.account,
            downloaded_real: Some(download_stats.raw),
            seeding: Some(tracker_stats.seeding),
            leeching: Some(tracker_stats.leeching),
            snatched: Some(tracker_stats.snatched),
            bonus_points: Some(yen_stats.current),
            bonus_points_per_hour: Some(yen_stats.per_hour),
            seed_size: Some(tracker_stats.seed_size),
            average_seed_time: Some(tracker_stats.avg_seed_time),
            uploaded_torrents: Some(torrents_stats.uploaded),
            ..Default::default()
        }
    }
}

#[async_trait]
impl Scraper for AnimeBytesScraper {
    async fn scrape(
        &self,
        indexer: Indexer,
        client: &reqwest::Client,
    ) -> Result<UserProfileScraped> {
        let res = client
            .get("https://animebytes.tv/api/stats/personal")
            .header(
                "Authorization",
                indexer
                    .auth_data
                    .get("api_key")
                    .ok_or("AB API key not found")
                    .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?
                    .get("value")
                    .ok_or("AB API key value not found")
                    .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?
                    .as_str()
                    .unwrap(),
            )
            .send()
            .await
            .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?;

        let body = res.text().await.unwrap();
        let response = serde_json::from_str::<AnimeBytesResponse>(&body)
            .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?;

        if !response.success {
            return Err(Error::CouldNotScrapeIndexer(response.error.unwrap_or(body)));
        }

        Ok(response.into())
    }
}
