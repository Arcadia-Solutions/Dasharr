use crate::{Dasharr, models::user_stats::UserProfileVec};
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use serde::Deserialize;
use std::collections::HashMap;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct GetUserStatsQueryParams {
    #[param(value_type = String, format = DateTime)]
    pub date_from: NaiveDateTime,
    #[param(value_type = String, format = DateTime)]
    pub date_to: NaiveDateTime,
}

#[derive(Debug)]
struct GetUserStatsQuery {
    pub indexer_ids: Vec<i64>,
    pub date_from: NaiveDateTime,
    pub date_to: NaiveDateTime,
}

fn parse_query_string(
    req: &HttpRequest,
) -> std::result::Result<GetUserStatsQuery, actix_web::Error> {
    let query_string = req.query_string();
    let mut indexer_ids = Vec::new();
    let mut date_from: Option<NaiveDateTime> = None;
    let mut date_to: Option<NaiveDateTime> = None;

    for (key, value) in url::form_urlencoded::parse(query_string.as_bytes()) {
        match key.as_ref() {
            "indexer_ids" => {
                if let Ok(id) = value.parse::<i64>() {
                    indexer_ids.push(id);
                }
            }
            "date_from" => {
                if let Ok(dt) = NaiveDateTime::parse_from_str(&value, "%Y-%m-%dT%H:%M:%S%.f") {
                    date_from = Some(dt);
                } else if let Ok(dt) = NaiveDateTime::parse_from_str(&value, "%Y-%m-%dT%H:%M:%S") {
                    date_from = Some(dt);
                }
            }
            "date_to" => {
                if let Ok(dt) = NaiveDateTime::parse_from_str(&value, "%Y-%m-%dT%H:%M:%S%.f") {
                    date_to = Some(dt);
                } else if let Ok(dt) = NaiveDateTime::parse_from_str(&value, "%Y-%m-%dT%H:%M:%S") {
                    date_to = Some(dt);
                }
            }
            _ => {}
        }
    }

    Ok(GetUserStatsQuery {
        indexer_ids,
        date_from: date_from
            .ok_or_else(|| actix_web::error::ErrorBadRequest("missing date_from"))?,
        date_to: date_to.ok_or_else(|| actix_web::error::ErrorBadRequest("missing date_to"))?,
    })
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
    req: HttpRequest,
) -> std::result::Result<HttpResponse, actix_web::Error> {
    let query = parse_query_string(&req)?;

    if query.indexer_ids.is_empty() {
        return Err(actix_web::error::ErrorBadRequest("indexer_ids is required"));
    }

    let user_stats = arc
        .pool
        .find_user_stats(&query.indexer_ids, &query.date_from, &query.date_to)
        .await
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
        })?;

    let mut grouped_profiles: HashMap<i32, Vec<crate::models::user_stats::UserProfile>> =
        HashMap::new();
    for profile in user_stats {
        grouped_profiles
            .entry(profile.indexer_id)
            .or_insert_with(Vec::new)
            .push(profile);
    }

    let grouped_stats: HashMap<i32, UserProfileVec> = grouped_profiles
        .into_iter()
        .map(|(indexer_id, profiles)| (indexer_id, UserProfileVec::from_vec(profiles)))
        .collect();

    Ok(HttpResponse::Ok().json(grouped_stats))
}
