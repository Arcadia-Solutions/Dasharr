use chrono::NaiveDateTime;

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
                        class, rank_uploaded, donor, seeding, leeching, rank_downloaded, rank_uploads,
                        rank_requests, rank_bounty, rank_posts, rank_artists, rank_overall,
                        paranoia_text, warned, posts, torrent_comments, collages_started, collages_contrib,
                        requests_filled, requests_voted, uploaded_torrents, groups, snatched, invited, bonus_points,
                        bonus_points_per_hour, uploaded_real, downloaded_real, seed_size, average_seed_time
                    )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17,
                    $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36)
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
                profile.profile.seeding,
                profile.profile.leeching,
                profile.profile.rank_downloaded,
                profile.profile.rank_uploads,
                profile.profile.rank_requests,
                profile.profile.rank_bounty,
                profile.profile.rank_posts,
                profile.profile.rank_artists,
                profile.profile.rank_overall,
                profile.profile.paranoia_text,
                profile.profile.warned,
                profile.profile.posts,
                profile.profile.torrent_comments,
                profile.profile.collages_started,
                profile.profile.collages_contrib,
                profile.profile.requests_filled,
                profile.profile.requests_voted,
                profile.profile.uploaded_torrents,
                profile.profile.groups,
                profile.profile.snatched,
                profile.profile.invited,
                profile.profile.bonus_points,
                profile.profile.bonus_points_per_hour,
                profile.profile.uploaded_real,
                profile.profile.downloaded_real,
                profile.profile.seed_size,
                profile.profile.average_seed_time,
            )
            // .execute(&mut *tx)
            .execute(self.borrow())
            .await
            .map_err(|e| Error::CouldNotInsertStats(e.to_string()))?;
        }

        // tx.commit().await?;

        Ok(())
    }

    pub async fn find_user_stats(
        &self,
        indexer_id: i64,
        date_from: &NaiveDateTime,
        date_to: &NaiveDateTime,
    ) -> Result<Vec<UserProfile>> {
        // currently not possible to use a macro
        // https://github.com/launchbadge/sqlx/issues/514
        // query_as_unchecked also tries to check the result of the query...
        let indexers: Vec<UserProfile> = sqlx::query_as(
            r#"
            SELECT * FROM user_profiles
            WHERE indexer_id = $1 AND scraped_at BETWEEN  $2 AND $3
            ORDER BY scraped_at ASC
            "#,
        )
        .bind(indexer_id)
        .bind(date_from)
        .bind(date_to)
        .fetch_all(self.borrow())
        .await
        .map_err(Error::CouldNotGetIndexers)?;

        Ok(indexers)
    }
}
