use async_trait::async_trait;
use serde::Deserialize;

use crate::models::{
    indexer::{Indexer, Scraper},
    user_stats::UserProfileScraped,
};

pub struct RedactedScraper;

#[derive(Debug, Deserialize)]
struct RedactedResponse {
    response: UserProfileScrapedContent,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsonStats {
    // last_access: NaiveDateTime,
    uploaded: u64,
    downloaded: u64,
    ratio: f32,
    required_ratio: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsonRanks {
    #[serde(rename = "uploaded")]
    rank_uploaded: u32,
    #[serde(rename = "downloaded")]
    rank_downloaded: u32,
    uploads: u32,
    requests: u32,
    bounty: u32,
    posts: u32,
    artists: u32,
    overall: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsonPersonal {
    class: String,
    paranoia: u32,
    paranoia_text: String,
    donor: bool,
    warned: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsonCommunity {
    posts: u32,
    torrent_comments: u32,
    collages_started: u32,
    collages_contrib: u32,
    requests_filled: u32,
    requests_voted: u32,
    #[serde(rename = "uploaded")]
    uploaded_torrents: u32,
    groups: u32,
    seeding: u32,
    leeching: u32,
    snatched: u32,
    invited: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserProfileScrapedContent {
    avatar: String,
    stats: JsonStats,
    ranks: JsonRanks,
    personal: JsonPersonal,
    community: JsonCommunity,
}

impl From<UserProfileScrapedContent> for UserProfileScraped {
    fn from(wrapper: UserProfileScrapedContent) -> Self {
        UserProfileScraped {
            avatar: wrapper.avatar,
            // last_access: wrapper.stats.last_access,
            uploaded: wrapper.stats.uploaded,
            downloaded: wrapper.stats.downloaded,
            ratio: wrapper.stats.ratio,
            required_ratio: wrapper.stats.required_ratio,
            rank_uploaded: Some(wrapper.ranks.rank_uploaded),
            rank_downloaded: Some(wrapper.ranks.rank_downloaded),
            rank_uploads: Some(wrapper.ranks.uploads),
            rank_requests: Some(wrapper.ranks.requests),
            rank_bounty: Some(wrapper.ranks.bounty),
            rank_posts: Some(wrapper.ranks.posts),
            rank_artists: Some(wrapper.ranks.artists),
            rank_overall: Some(wrapper.ranks.overall),
            class: wrapper.personal.class,
            paranoia: Some(wrapper.personal.paranoia),
            paranoia_text: Some(wrapper.personal.paranoia_text),
            donor: Some(wrapper.personal.donor),
            warned: Some(wrapper.personal.warned),
            posts: Some(wrapper.community.posts),
            torrent_comments: Some(wrapper.community.torrent_comments),
            collages_started: Some(wrapper.community.collages_started),
            collages_contrib: Some(wrapper.community.collages_contrib),
            requests_filled: Some(wrapper.community.requests_filled),
            requests_voted: Some(wrapper.community.requests_voted),
            uploaded_torrents: Some(wrapper.community.uploaded_torrents),
            groups: Some(wrapper.community.groups),
            seeding: Some(wrapper.community.seeding),
            leeching: Some(wrapper.community.leeching),
            snatched: Some(wrapper.community.snatched),
            invited: Some(wrapper.community.invited),
        }
    }
}

#[async_trait]
impl Scraper for RedactedScraper {
    async fn scrape(
        &self,
        indexer: Indexer,
    ) -> Result<UserProfileScraped, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let res = client
            .get(format!(
                "https://redacted.sh/ajax.php?action=user&id={}",
                indexer
                    .auth_data
                    .get("user_id")
                    .ok_or("redacted user_id not found")?
                    .get("value")
                    .ok_or("redacted user_id value not found")?
                    .as_str()
                    .unwrap()
            ))
            .header(
                "Authorization",
                indexer
                    .auth_data
                    .get("api_key")
                    .ok_or("redacted API key not found")?
                    .get("value")
                    .ok_or("redacted API key value not found")?
                    .as_str()
                    .unwrap(),
            )
            .send()
            .await?;

        let body = res.text().await?;
        let profile: UserProfileScraped = serde_json::from_str::<RedactedResponse>(&body)?
            .response
            .into();

        Ok(profile)
    }
}
