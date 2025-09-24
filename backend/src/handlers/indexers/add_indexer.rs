use crate::{
    Dasharr,
    error::Result,
    models::indexer::{Indexer, NewIndexer},
};
use actix_web::{
    HttpResponse,
    web::{Data, Json},
};

#[utoipa::path(
    post,
    operation_id = "Add indexer",
    tag = "Indexers",
    path = "/api/indexers",
    responses(
        (status = 201, description = "Successfully added the indexer", body=Indexer),
    )
)]
pub async fn exec(arc: Data<Dasharr>, new_indexer: Json<NewIndexer>) -> Result<HttpResponse> {
    let created_indexer = arc.pool.create_indexer(&new_indexer).await?;

    Ok(HttpResponse::Created().json(created_indexer))
}
