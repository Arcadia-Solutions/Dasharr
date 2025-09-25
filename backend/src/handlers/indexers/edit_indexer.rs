use crate::{
    Dasharr,
    error::Result,
    models::indexer::{Indexer, UpdatedIndexer},
};
use actix_web::{
    HttpResponse,
    web::{Data, Json},
};

#[utoipa::path(
    put,
    operation_id = "Add indexer",
    tag = "Indexers",
    path = "/api/indexers",
    responses(
        (status = 201, description = "Successfully edited the indexer", body=Indexer),
    )
)]
pub async fn exec(arc: Data<Dasharr>, new_indexer: Json<UpdatedIndexer>) -> Result<HttpResponse> {
    let created_indexer = arc.pool.update_indexer(&new_indexer).await?;

    Ok(HttpResponse::Created().json(created_indexer))
}
