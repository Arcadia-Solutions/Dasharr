use crate::{
    connection_pool::ConnectionPool,
    error::{Error, Result},
    models::indexer::{Indexer, IndexerLite, UpdatedIndexer},
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

    pub async fn find_indexers_lite(&self) -> Result<Vec<IndexerLite>> {
        let indexers = sqlx::query_as!(
            IndexerLite,
            r#"
                SELECT id, name, enabled FROM indexers
            "#
        )
        .fetch_all(self.borrow())
        .await
        .map_err(Error::CouldNotGetIndexers)?;

        Ok(indexers)
    }

    pub async fn find_indexers_lite_with_available_data(&self) -> Result<Vec<IndexerLite>> {
        let indexers = sqlx::query_as!(
            IndexerLite,
            r#"
            SELECT DISTINCT i.id, i.name, i.enabled
            FROM indexers i
            INNER JOIN user_profiles up ON i.id = up.indexer_id
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
