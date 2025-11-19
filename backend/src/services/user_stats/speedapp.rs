use serde::Deserialize;

use async_trait::async_trait;

use crate::{
    error::{Error, Result},
    models::{
        indexer::{Indexer, Scraper},
        user_stats::UserProfileScraped,
    },
};

#[derive(Debug, Deserialize)]
struct SpeedappResponse {
    error: Option<bool>,
    message: Option<String>,
    // id: Option<i64>,
    // username: Option<String>,
    // email: Option<String>,
    // created_at: Option<String>,
    // class: Option<i64>,
    // avatar: Option<String>,
    uploaded: Option<i64>,
    downloaded: Option<i64>,
    // title: Option<String>,
    is_donor: Option<bool>,
    warned: Option<bool>,
    // passkey: Option<String>,
    // invites: Option<i64>,
    // timezone: Option<String>,
    // hit_and_run_count: Option<i64>,
    snatch_count: Option<i32>,
    // need_seed: Option<i64>,
    average_seed_time: Option<i64>,
    // locale: Option,
    // free_leech_tokens: Option<i64>,
    // double_upload_tokens: Option<i64>,
}

impl From<SpeedappResponse> for UserProfileScraped {
    fn from(wrapper: SpeedappResponse) -> Self {
        let uploaded = wrapper.uploaded.unwrap_or(0);
        let downloaded = wrapper.downloaded.unwrap_or(0);
        let ratio = if downloaded == 0 && uploaded > 0 {
            f32::MAX
        } else {
            uploaded as f32 / downloaded as f32
        };

        UserProfileScraped {
            uploaded,
            downloaded,
            ratio,
            // class: wrapper.class.unwrap_or(0).to_string(),
            donor: wrapper.is_donor,
            warned: wrapper.warned,
            average_seed_time: wrapper.average_seed_time,
            snatched: wrapper.snatch_count,
            ..Default::default()
        }
    }
}

pub struct SpeedappScraper;

#[async_trait]
impl Scraper for SpeedappScraper {
    async fn scrape(
        &self,
        indexer: Indexer,
        client: &reqwest::Client,
    ) -> Result<UserProfileScraped> {
        let res = client
            .get("https://speedapp.io/api/me")
            .header(
                "Authorization",
                format!(
                    "Bearer {}",
                    indexer
                        .auth_data
                        .get("api_key")
                        .ok_or("Speedapp api key not found.")
                        .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
                        .get("value")
                        .ok_or("Speedapp api key value not found")
                        .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
                        .as_str()
                        .unwrap()
                ),
            )
            .send()
            .await
            .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?;

        let body = res.text().await.unwrap();
        let response = serde_json::from_str::<SpeedappResponse>(&body)
            .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?;

        if response.error.unwrap_or(false) {
            return Err(Error::CouldNotScrapeIndexer(response.message.unwrap()));
        }

        Ok(response.into())
    }
}
