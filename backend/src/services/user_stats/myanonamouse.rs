use async_trait::async_trait;
use serde::{Deserialize, Deserializer, de::Visitor};
use std::fmt;

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
    #[serde(deserialize_with = "deserialize_ratio")]
    pub ratio: f32,
    pub seedbonus: i64,
}

struct RatioVisitor;

impl<'de> Visitor<'de> for RatioVisitor {
    type Value = f32;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an f32 or the character '∞'")
    }

    fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v as f32)
    }

    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v == "∞" {
            Ok(f32::MAX)
        } else {
            v.parse::<f32>()
                .map_err(|_| E::custom(format!("Expected an f32 or '∞', found: {}", v)))
        }
    }
}

fn deserialize_ratio<'de, D>(deserializer: D) -> std::result::Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(RatioVisitor)
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

        let body = res
            .text()
            .await
            .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?;
        let response = serde_json::from_str::<MyAnonamouseResponse>(&body).map_err(|e| {
            Error::CouldNotScrapeIndexer(format!("Your mam id is probably invalid. {}", e))
        })?;

        Ok(response.into())
    }
}
