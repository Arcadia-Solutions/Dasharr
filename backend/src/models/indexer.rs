use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

use crate::{
    error::{Error, Result},
    models::user_stats::UserProfileScraped,
    services::user_stats::{
        aither::AitherScraper, anime_bytes::AnimeBytesScraper, anthelion::AnthelionScraper,
        blutopia::BlutopiaScraper, broadcasthenet::BroadcasthenetScraper,
        gazelle_games::GazelleGamesScraper, lst::LSTScraper, oldtoons::OldToonsScraper,
        orpheus::OrpheusScraper, phoenix_project::PhoenixProjectScraper, redacted::RedactedScraper,
        reel_flix::ReelFlixScraper,
    },
};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthItem {
    pub value: String,
    pub explanation: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct Indexer {
    pub id: i32,
    pub enabled: bool,
    pub name: String,
    #[schema(value_type = HashMap<String, AuthItem>)]
    pub auth_data: Value,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct IndexerEnriched {
    pub id: i32,
    pub enabled: bool,
    pub name: String,
    pub error: Option<String>,
    #[schema(value_type = String, format = DateTime)]
    pub last_scraped_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdatedIndexer {
    pub id: i32,
    #[schema(value_type = HashMap<String, AuthItem>)]
    pub auth_data: Value,
}

#[async_trait]
pub trait Scraper {
    async fn scrape(&self, indexer: Indexer) -> Result<UserProfileScraped>;
}

impl Indexer {
    pub async fn scrape(self) -> Result<UserProfileScraped> {
        let scraper_ref: &dyn Scraper = match self.name.as_str() {
            "Redacted" => {
                static REDACTED_SCRAPER: RedactedScraper = RedactedScraper;
                &REDACTED_SCRAPER
            }
            "Orpheus" => {
                static ORPHEUS_SCRAPER: OrpheusScraper = OrpheusScraper;
                &ORPHEUS_SCRAPER
            }
            "GazelleGames" => {
                static GAZELLE_GAMES_SCRAPER: GazelleGamesScraper = GazelleGamesScraper;
                &GAZELLE_GAMES_SCRAPER
            }
            "Broadcasthenet" => {
                static BROADCASTHENET_SCRAPER: BroadcasthenetScraper = BroadcasthenetScraper;
                &BROADCASTHENET_SCRAPER
            }
            "Anthelion" => {
                static ANTHELION_SCRAPER: AnthelionScraper = AnthelionScraper;
                &ANTHELION_SCRAPER
            }
            "PhoenixProject" => {
                static PHOENIX_PROJECT_SCRAPER: PhoenixProjectScraper = PhoenixProjectScraper;
                &PHOENIX_PROJECT_SCRAPER
            }
            "AnimeBytes" => {
                static ANIME_BYTES_SCRAPER: AnimeBytesScraper = AnimeBytesScraper;
                &ANIME_BYTES_SCRAPER
            }
            "Blutopia" => {
                static BLUTOPIA_SCRAPER: BlutopiaScraper = BlutopiaScraper;
                &BLUTOPIA_SCRAPER
            }
            "Aither" => {
                static AITHER_SCRAPER: AitherScraper = AitherScraper;
                &AITHER_SCRAPER
            }
            "LST" => {
                static LST_SCRAPER: LSTScraper = LSTScraper;
                &LST_SCRAPER
            }
            "OldToons" => {
                static OLD_TOONS_SCRAPER: OldToonsScraper = OldToonsScraper;
                &OLD_TOONS_SCRAPER
            }
            "ReelFlix" => {
                static REEL_FLIX_SCRAPER: ReelFlixScraper = ReelFlixScraper;
                &REEL_FLIX_SCRAPER
            }
            _ => {
                return Err(Error::CouldNotScrapeIndexer(
                    "indexer has no scraper".into(),
                ));
            }
        };

        let profile = scraper_ref.scrape(self).await?; //.map_err(|e| Error::CouldNotScrapeIndexer(format!("{}", e.into())))?;
        Ok(profile)
    }
}
