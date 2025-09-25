use actix_web::web::Data;
use futures::StreamExt;

use crate::{Dasharr, error::Result};

pub async fn scrape_indexers(arc: &Data<Dasharr>) -> Result<()> {
    let indexers = arc.pool.find_indexers().await?;

    futures::stream::iter(indexers.iter())
        .for_each_concurrent(None, |indexer| async {
            if let Err(e) = indexer.clone().scrape().await {
                eprintln!("Error scraping {}: {}", indexer.name, e);
            }
        })
        .await;

    Ok(())
}
