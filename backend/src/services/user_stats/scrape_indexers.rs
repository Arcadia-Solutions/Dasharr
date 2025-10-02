use std::sync::Arc;

use chrono::Local;
use futures::StreamExt;

use crate::{connection_pool::ConnectionPool, models::user_stats::UserProfile};

pub struct ScraperError {
    pub message: String,
    pub indexer_id: i32,
}

pub async fn scrape_indexers(pool: &Arc<ConnectionPool>) -> crate::error::Result<()> {
    let indexers = pool.find_indexers().await?;

    type ScrapeResult = Result<UserProfile, ScraperError>;

    let results = futures::stream::iter(indexers.into_iter())
        .filter(|indexer| futures::future::ready(indexer.enabled))
        .map(|indexer| async move {
            log::info!("scraping indexer: {}", indexer.name);
            let indexer_id = indexer.id;
            match indexer.scrape().await {
                Ok(profile) => Ok(UserProfile {
                    profile,
                    scraped_at: Local::now(),
                    indexer_id,
                }),
                Err(e) => Err(ScraperError {
                    message: e.to_string(),
                    indexer_id,
                }),
            }
        })
        .buffer_unordered(10)
        .collect::<Vec<ScrapeResult>>()
        .await;

    let mut profiles = Vec::new();
    let mut errors = Vec::new();

    for result in results {
        match result {
            Ok(profile) => profiles.push(profile),
            Err(e) => errors.push(e),
        }
    }

    pool.create_stats(&profiles).await?;
    // also sets the current error to null if there's no issue
    // so this needs to be called even without errors
    pool.update_indexers_status(&errors).await?;

    Ok(())
}
