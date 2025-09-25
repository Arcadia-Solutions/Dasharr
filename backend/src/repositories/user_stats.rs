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
                profile.base.avatar,
                // profile.base.last_access,
                profile.base.uploaded,
                profile.base.downloaded,
                profile.base.ratio,
                profile.base.required_ratio,
                profile.base.class,
                profile.base.rank_uploaded,
                profile.base.donor,
            )
            // .execute(&mut *tx)
            .execute(self.borrow())
            .await
            .map_err(|e| Error::CouldNotInsertStats(e.to_string()))?;
        }

        // tx.commit().await?;

        Ok(())
    }
}
