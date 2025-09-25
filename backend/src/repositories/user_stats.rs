use crate::{
    connection_pool::ConnectionPool,
    error::{Error, Result},
    models::user_stats::UserProfile,
};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn create_stats(&self, user_profiles: &Vec<UserProfile>) -> Result<()> {
        // let mut tx = <ConnectionPool as Borrow<PgPool>>::borrow(self)
        //     .begin()
        //     .await?;

        for profile in user_profiles {
            sqlx::query!(
                r#"
                    INSERT INTO user_profiles (
                        indexer_id, avatar, uploaded, downloaded, ratio, required_ratio,
                        class, rank_uploaded, donor
                    )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                    "#,
                profile.indexer_id,
                profile.profile.avatar,
                // profile.profile.last_access,
                profile.profile.uploaded,
                profile.profile.downloaded,
                profile.profile.ratio,
                profile.profile.required_ratio,
                profile.profile.class,
                profile.profile.rank_uploaded,
                profile.profile.donor,
            )
            // .execute(&mut *tx)
            .execute(self.borrow())
            .await
            .map_err(|e| Error::CouldNotInsertStats(e.to_string()))?;
        }

        // tx.commit().await?;

        Ok(())
    }

    pub async fn find_user_stats(&self, indexer_id: i64) -> Result<Vec<UserProfile>> {
        // currently not possible to use a macro
        // https://github.com/launchbadge/sqlx/issues/514
        // query_as_unchecked also tries to check the result of the query...
        let indexers: Vec<UserProfile> = sqlx::query_as(
            r#"
            SELECT * FROM user_profiles
            WHERE indexer_id = $1
            "#,
        )
        .bind(indexer_id)
        .fetch_all(self.borrow())
        .await
        .map_err(Error::CouldNotGetIndexers)?;

        Ok(indexers)
    }
}
