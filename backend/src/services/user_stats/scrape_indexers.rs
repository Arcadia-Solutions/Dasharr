use actix_web::web::Data;
use chrono::Local;
use futures::StreamExt;

use crate::{Dasharr, error::Result, models::user_stats::UserProfile};

pub async fn scrape_indexers(arc: &Data<Dasharr>) -> Result<()> {
    let indexers = arc.pool.find_indexers().await?;

    let profiles = futures::stream::iter(indexers.iter())
        .filter_map(|indexer| async move {
            if indexer.enabled {
                log::info!("scraping indexer: {}", indexer.name);
                match indexer.clone().scrape().await {
                    Ok(profile) => Some(UserProfile {
                        profile,
                        scraped_at: Local::now(),
                        indexer_id: indexer.id,
                    }),
                    Err(e) => {
                        eprintln!("Error scraping {}: {}", indexer.name, e);
                        None
                    }
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .await;

    arc.pool.create_stats(&profiles).await?;

    Ok(())
}
