use crate::{Dasharr, models::user_stats::UserProfileVec};
use actix_web::web::{Data, Query};
use actix_web::HttpResponse;
use chrono::NaiveDateTime;
use serde::Deserialize;
use std::collections::HashMap;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct GetUserStatsQueryParams {
    pub indexer_ids: Vec<i64>,

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
    params(GetUserStatsQueryParams),
    responses(
        (status = 200, description = "Successfully got user stats", body=HashMap<i32, UserProfileVec>),
    )
)]
pub async fn exec(
    arc: Data<Dasharr>,
    query: Query<GetUserStatsQueryParams>,
) -> std::result::Result<HttpResponse, actix_web::Error> {
    let query = query.into_inner();

    if query.indexer_ids.is_empty() {
        return Err(actix_web::error::ErrorBadRequest("indexer_ids is required"));
    }

    let user_stats = arc
        .pool
        .find_user_stats(&query.indexer_ids, &query.date_from, &query.date_to)
        .await?;

    let mut grouped_profiles: HashMap<i32, Vec<crate::models::user_stats::UserProfile>> =
        HashMap::new();
    for profile in user_stats {
        grouped_profiles
            .entry(profile.indexer_id)
            .or_default()
            .push(profile);
    }

    let grouped_stats: HashMap<i32, UserProfileVec> = grouped_profiles
        .into_iter()
        .map(|(indexer_id, profiles)| (indexer_id, UserProfileVec::from_vec(profiles)))
        .collect();

    Ok(HttpResponse::Ok().json(grouped_stats))
}
