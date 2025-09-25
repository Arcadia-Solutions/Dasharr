use crate::{
    connection_pool::ConnectionPool,
    error::{Error, Result},
    models::indexer::{Indexer, UpdatedIndexer},
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
}
