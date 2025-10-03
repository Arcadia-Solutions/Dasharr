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
    operation_id = "Edit indexer",
    tag = "Indexers",
    path = "/api/indexers",
    responses(
        (status = 201, description = "Successfully edited the indexer", body=Indexer),
    )
)]
pub async fn exec(arc: Data<Dasharr>, indexer: Json<UpdatedIndexer>) -> Result<HttpResponse> {
    let updated_indexer = arc.pool.update_indexer(&indexer).await?;
    updated_indexer.clone().scrape().await?;

    Ok(HttpResponse::Created().json(updated_indexer))
}
