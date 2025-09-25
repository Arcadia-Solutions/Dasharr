use crate::{Dasharr, error::Result, models::indexer::Indexer};
use actix_web::{HttpResponse, web::Data};

#[utoipa::path(
    get,
    operation_id = "Get indexers",
    tag = "Indexers",
    path = "/api/indexers",
    responses(
        (status = 200, description = "Successfully got the indexer", body=Vec<Indexer>),
    )
)]
pub async fn exec(arc: Data<Dasharr>) -> Result<HttpResponse> {
    let indexers = arc.pool.find_indexers().await?;

    Ok(HttpResponse::Ok().json(indexers))
}
