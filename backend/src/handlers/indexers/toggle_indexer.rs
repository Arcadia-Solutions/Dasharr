use crate::{Dasharr, error::Result};
use actix_web::{
    HttpResponse,
    web::{Data, Path},
};

#[utoipa::path(
    put,
    operation_id = "Toggle indexer",
    tag = "Indexers",
    params(("id" = i32, Path, description = "Indexer id")),
    path = "/api/indexers/{id}/toggle",
    responses(
        (status = 201, description = "Successfully toggled the indexer"),
    )
)]
pub async fn exec(arc: Data<Dasharr>, id: Path<i32>) -> Result<HttpResponse> {
    arc.pool.toggle_indexer(id.into_inner()).await?;

    Ok(HttpResponse::Ok().json("{}"))
}
