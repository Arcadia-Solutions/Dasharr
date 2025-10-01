use crate::{Dasharr, error::Result, models::indexer::AuthItem};
use actix_web::{
    HttpResponse,
    web::{Data, Path},
};

#[utoipa::path(
    get,
    operation_id = "Get indexer's auth data",
    tag = "Indexers",
    path = "/api/indexers/{id}/auth-data",
    responses(
        (status = 200, description = "Successfully got the indexer's auth data", body=HashMap<String, AuthItem>),
    )
)]
pub async fn exec(arc: Data<Dasharr>, id: Path<i32>) -> Result<HttpResponse> {
    let indexers = arc.pool.find_indexer_auth_data(id.into_inner()).await?;

    Ok(HttpResponse::Ok().json(indexers))
}
