use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

use crate::{
    models::user_stats::UserProfileScraped,
    services::user_stats::{gazelle_games::GazelleGamesScraper, redacted::RedactedScraper},
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
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct IndexerLite {
    pub id: i32,
    pub enabled: bool,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdatedIndexer {
    pub id: i32,
    #[schema(value_type = HashMap<String, AuthItem>)]
    pub auth_data: Value,
}

#[async_trait]
pub trait Scraper {
    async fn scrape(
        &self,
        indexer: Indexer,
    ) -> Result<UserProfileScraped, Box<dyn std::error::Error>>;
}

#[derive(Debug)]
pub enum ScraperError {
    ScraperNotFound(String),
}

impl std::fmt::Display for ScraperError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ScraperError::ScraperNotFound(name) => {
                write!(f, "scraper not found for name: {}", name)
            }
        }
    }
}

impl std::error::Error for ScraperError {}

impl Indexer {
    pub async fn scrape(self) -> Result<UserProfileScraped, Box<dyn std::error::Error>> {
        let scraper_ref: &dyn Scraper = match self.name.as_str() {
            "Redacted" => {
                static REDACTED_SCRAPER: RedactedScraper = RedactedScraper;
                &REDACTED_SCRAPER
            }
            "GazelleGames" => {
                static GAZELLE_GAMES_SCRAPER: GazelleGamesScraper = GazelleGamesScraper;
                &GAZELLE_GAMES_SCRAPER
            }
            _ => {
                return Err(Box::new(ScraperError::ScraperNotFound(self.name.clone())));
            }
        };

        let profile = scraper_ref.scrape(self).await?;
        Ok(profile)
    }
}
