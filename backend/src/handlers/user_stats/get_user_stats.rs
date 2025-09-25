use crate::{Dasharr, error::Result, models::user_stats::UserProfileVec};
use actix_web::{
    HttpResponse,
    web::{Data, Query},
};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetUserStatsQueryId {
    pub indexer_id: i64,
}

#[utoipa::path(
    get,
    operation_id = "Get user stats",
    tag = "User stats",
    path = "/api/user-stats",
    params(GetUserStatsQueryId),
    responses(
        (status = 200, description = "Successfully got user stats", body=UserProfileVec),
    )
)]
pub async fn exec(arc: Data<Dasharr>, query: Query<GetUserStatsQueryId>) -> Result<HttpResponse> {
    let user_stats = arc.pool.find_user_stats(query.indexer_id).await?;
    // let test: UserProfileVec = user_stats.into();
    let user_stats_reduced = UserProfileVec::from_vec(user_stats);

    Ok(HttpResponse::Ok().json(user_stats_reduced))
}
