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
struct DigitalCoreResponse {
    bonuspoang: f64,
    downloaded: i64,
    downloaded_real: i64,
    uploaded: i64,
    uploaded_real: i64,
    // leechbonus: f32,
    #[serde(rename = "peersSeeder")]
    peers_seeder: i32,
    #[serde(rename = "peersLeecher")]
    peers_leecher: i32,
    warned: String,
    #[serde(rename = "torrentComments")]
    torrent_comments: i32,
    donor: String,
    #[serde(rename = "forumPosts")]
    forum_posts: i32,
    invitees: i32,
}

impl From<DigitalCoreResponse> for UserProfileScraped {
    fn from(wrapper: DigitalCoreResponse) -> Self {
        let ratio = if wrapper.downloaded == 0 && wrapper.uploaded > 0 {
            f32::MAX
        } else {
            wrapper.uploaded as f32 / wrapper.downloaded as f32
        };

        UserProfileScraped {
            uploaded: wrapper.uploaded,
            downloaded: wrapper.downloaded,
            ratio,
            donor: Some(wrapper.donor.eq("yes")),
            warned: Some(wrapper.warned.eq("yes")),
            seeding: Some(wrapper.peers_seeder),
            leeching: Some(wrapper.peers_leecher),
            bonus_points: Some(wrapper.bonuspoang as i64),
            uploaded_real: Some(wrapper.uploaded_real),
            downloaded_real: Some(wrapper.downloaded_real),
            torrent_comments: Some(wrapper.torrent_comments),
            posts: Some(wrapper.forum_posts),
            invited: Some(wrapper.invitees),
            ..Default::default()
        }
    }
}

pub struct DigitalCoreScraper;

#[async_trait]
impl Scraper for DigitalCoreScraper {
    async fn scrape(
        &self,
        indexer: Indexer,
        client: &reqwest::Client,
    ) -> Result<UserProfileScraped> {
        let cookies = indexer
            .auth_data
            .get("cookies")
            .ok_or("DigitalCore cookies not found.")
            .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
            .get("value")
            .ok_or("DigitalCore cookies not found.")
            .map_err(|e| Error::CouldNotScrapeIndexer(e.into()))?
            .as_str()
            .unwrap();

        if !cookies.contains("uid=") {
            return Err(Error::CouldNotScrapeIndexer(
                "Cannot find cookie for 'uid'.".to_string(),
            ));
        }

        if !cookies.contains("pass=") {
            return Err(Error::CouldNotScrapeIndexer(
                "Cannot find cookie for 'pass'.".to_string(),
            ));
        }

        let uid_cookie = cookies.split("uid=").collect::<Vec<&str>>()[1]
            .split(";")
            .collect::<Vec<&str>>()[0];

        let res = client
            .get(format!("https://digitalcore.club/api/v1/users/{}", uid_cookie))
            .header(
                "Cookie",
                cookies
            )
            .send()
            .await
            .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?;

        let body = res.text().await.unwrap();
        let response = serde_json::from_str::<DigitalCoreResponse>(&body)
            .map_err(|e| Error::CouldNotScrapeIndexer(e.to_string()))?;

        Ok(response.into())
    }
}
