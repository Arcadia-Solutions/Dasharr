use async_trait::async_trait;
use serde::Deserialize;

use crate::{
    error::{Error, Result},
    models::{
        indexer::{Indexer, Scraper},
        user_stats::UserProfileScraped,
    },
};

pub struct MyAnonamouseScraper;

#[derive(Debug, Deserialize)]
pub struct MyAnonamouseResponse {
    pub classname: String,
    pub uploaded_bytes: i64,
    pub downloaded_bytes: i64,
    pub ratio: f32,
    pub seedbonus: i64,
}

impl From<MyAnonamouseResponse> for UserProfileScraped {
    fn from(wrapper: MyAnonamouseResponse) -> Self {
        UserProfileScraped {
            uploaded: wrapper.uploaded_bytes,
            downloaded: wrapper.downloaded_bytes,
            class: wrapper.classname,
            bonus_points: Some(wrapper.seedbonus),
            ratio: wrapper.ratio,
            ..Default::default()
        }
    }
}

#[async_trait]
impl Scraper for MyAnonamouseScraper {
    async fn scrape(
        &self,
        indexer: Indexer,
        client: &reqwest::Client,
    ) -> Result<UserProfileScraped> {
        let res = client
            .get("https://www.myanonamouse.net/jsonLoad.php")
            .header(
                "cookie",
                format!(
                    "mam_id={}",
                    indexer
                        .auth_data
                        .get("mam_id")
                        .ok_or("MAM id key not found")
                        .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
                        .get("value")
                        .ok_or("MAM id key value not found")
                        .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
                        .as_str()
                        .unwrap()
                ),
            )
            .send()
            .await
            .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?;

        let body = res.text().await.unwrap();
        let response = serde_json::from_str::<MyAnonamouseResponse>(&body).map_err(|e| {
            Error::CouldNotScrapeIndexer(format!("Your mam id is probably invalid. {}", e))
        })?;

        Ok(response.into())
    }
}
