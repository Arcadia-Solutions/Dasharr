use crate::{
    connection_pool::ConnectionPool,
    error::{Error, Result},
    models::indexer::{Indexer, NewIndexer},
};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn create_indexer(&self, new_indexer: &NewIndexer) -> Result<Indexer> {
        let created_indexer = sqlx::query_as!(
            Indexer,
            r#"
                INSERT INTO indexers (name, api_key)
                VALUES ($1, $2)
                RETURNING *
            "#,
            new_indexer.name,
            new_indexer.api_key
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotCreateIndexer)?;

        Ok(created_indexer)
    }
}
