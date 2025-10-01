use serde_json::Value;

use crate::{
    connection_pool::ConnectionPool,
    error::{Error, Result},
    models::indexer::{Indexer, IndexerEnriched, UpdatedIndexer},
};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn update_indexer(&self, indexer: &UpdatedIndexer) -> Result<Indexer> {
        let updated_indexer = sqlx::query_as!(
            Indexer,
            r#"
                UPDATE indexers
                SET auth_data = $2
                WHERE id = $1
                RETURNING *
            "#,
            indexer.id,
            indexer.auth_data
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotCreateIndexer)?;

        Ok(updated_indexer)
    }

    pub async fn find_indexers(&self) -> Result<Vec<Indexer>> {
        let indexers = sqlx::query_as!(
            Indexer,
            r#"
                SELECT * FROM indexers
            "#
        )
        .fetch_all(self.borrow())
        .await
        .map_err(Error::CouldNotGetIndexers)?;

        Ok(indexers)
    }

    pub async fn find_indexer_auth_data(&self, id: i32) -> Result<Value> {
        let auth_data_record = sqlx::query!(
            r#"
                SELECT auth_data FROM indexers WHERE id = $1
            "#,
            id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotGetIndexerAuthData)?;

        Ok(auth_data_record.auth_data)
    }

    pub async fn find_indexers_enriched(&self) -> Result<Vec<IndexerEnriched>> {
        let indexers = sqlx::query_as!(
            IndexerEnriched,
            r#"
            SELECT
                i.id,
                i.name,
                i.enabled,
                MAX(up.scraped_at) AS last_scraped_at
            FROM
                indexers AS i
            LEFT JOIN
                user_profiles AS up ON up.indexer_id = i.id
            GROUP BY
                i.id, i.name, i.enabled
            ORDER BY
                i.id;
            "#
        )
        .fetch_all(self.borrow())
        .await
        .map_err(Error::CouldNotGetIndexers)?;

        Ok(indexers)
    }

    pub async fn find_indexers_enriched_with_available_data(&self) -> Result<Vec<IndexerEnriched>> {
        let indexers = sqlx::query_as!(
            IndexerEnriched,
            r#"
            SELECT DISTINCT ON (i.id) i.id, i.name, i.enabled, up.scraped_at AS last_scraped_at
            FROM indexers AS i
            LEFT JOIN user_profiles AS up
              ON i.id = up.indexer_id
            ORDER BY i.id DESC;
            "#
        )
        .fetch_all(self.borrow())
        .await
        .map_err(Error::CouldNotGetIndexers)?;

        Ok(indexers)
    }

    pub async fn toggle_indexer(&self, indexer_id: i32) -> Result<()> {
        sqlx::query!(
            r#"
                UPDATE indexers
                SET enabled = NOT enabled
                WHERE id = $1
            "#,
            indexer_id
        )
        .execute(self.borrow())
        .await
        .map_err(Error::CouldNotToggleIndexer)?;

        Ok(())
    }
}
