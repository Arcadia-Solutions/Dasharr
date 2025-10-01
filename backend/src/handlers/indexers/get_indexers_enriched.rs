use crate::{Dasharr, error::Result, models::indexer::IndexerEnriched};
use actix_web::{
    HttpResponse,
    web::{Data, Query},
};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetIndexersEnrichedQuery {
    pub only_with_available_data: bool,
}

#[utoipa::path(
    get,
    operation_id = "Get indexers enriched",
    tag = "Indexers",
    path = "/api/indexers/enriched",
    params(GetIndexersEnrichedQuery),
    responses(
        (status = 200, description = "Successfully got the indexers (enriched)", body=Vec<IndexerEnriched>),
    )
)]
pub async fn exec(
    arc: Data<Dasharr>,
    query: Query<GetIndexersEnrichedQuery>,
) -> Result<HttpResponse> {
    let indexers = if query.only_with_available_data {
        arc.pool
            .find_indexers_enriched_with_available_data()
            .await?
    } else {
        arc.pool.find_indexers_enriched().await?
    };

    Ok(HttpResponse::Ok().json(indexers))
}
