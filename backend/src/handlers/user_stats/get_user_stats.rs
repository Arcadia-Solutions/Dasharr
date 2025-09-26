use crate::{Dasharr, error::Result, models::user_stats::UserProfileVec};
use actix_web::{
    HttpResponse,
    web::{Data, Query},
};
use chrono::NaiveDateTime;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct GetUserStatsQuery {
    pub indexer_id: i64,
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
        (status = 200, description = "Successfully got user stats", body=UserProfileVec),
    )
)]
pub async fn exec(arc: Data<Dasharr>, query: Query<GetUserStatsQuery>) -> Result<HttpResponse> {
    let user_stats = arc
        .pool
        .find_user_stats(query.indexer_id, &query.date_from, &query.date_to)
        .await?;
    // let test: UserProfileVec = user_stats.into();
    let user_stats_reduced = UserProfileVec::from_vec(user_stats);

    Ok(HttpResponse::Ok().json(user_stats_reduced))
}
