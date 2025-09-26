use crate::{Dasharr, error::Result, models::indexer::IndexerLite};
use actix_web::{
    HttpResponse,
    web::{Data, Query},
};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetIndexersLiteQuery {
    pub only_with_available_data: bool,
}

#[utoipa::path(
    get,
    operation_id = "Get indexers lite",
    tag = "Indexers",
    path = "/api/indexers/lite",
    params(GetIndexersLiteQuery),
    responses(
        (status = 200, description = "Successfully got the indexers (lite)", body=Vec<IndexerLite>),
    )
)]
pub async fn exec(arc: Data<Dasharr>, query: Query<GetIndexersLiteQuery>) -> Result<HttpResponse> {
    let indexers = if query.only_with_available_data {
        arc.pool.find_indexers_lite_with_available_data().await?
    } else {
        arc.pool.find_indexers_lite().await?
    };

    Ok(HttpResponse::Ok().json(indexers))
}
