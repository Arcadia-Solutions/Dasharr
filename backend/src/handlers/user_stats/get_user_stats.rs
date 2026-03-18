use crate::{Dasharr, error::Result, models::user_stats::IndexerStats};
use actix_web::{
    HttpResponse,
    web::{Data, Query},
};
use chrono::NaiveDateTime;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct GetUserStatsQuery {
    /// Comma-separated list of indexer IDs
    pub indexer_ids: String,
    #[param(value_type = String, format = DateTime)]
    pub date_from: NaiveDateTime,
    #[param(value_type = String, format = DateTime)]
    pub date_to: NaiveDateTime,
}

#[utoipa::path(
    get,
    operation_id = "Get user stats",
    tag = "User stats",
    path = "/api/user-stats",
    params(GetUserStatsQuery),
    responses(
        (status = 200, description = "Successfully got user stats", body=Vec<IndexerStats>),
    )
)]
pub async fn exec(arc: Data<Dasharr>, query: Query<GetUserStatsQuery>) -> Result<HttpResponse> {
    let indexer_ids: Vec<i32> = query
        .indexer_ids
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    let user_stats = arc
        .pool
        .find_user_stats(&indexer_ids, &query.date_from, &query.date_to)
        .await?;
    let grouped_stats = IndexerStats::group_by_indexer(user_stats);

    Ok(HttpResponse::Ok().json(grouped_stats))
}
