use crate::{Dasharr, error::Result, services::user_stats::scrape_indexers::scrape_indexers};
use actix_web::{HttpResponse, web::Data};

#[utoipa::path(
    get,
    operation_id = "Scrape user stats",
    tag = "User stats",
    path = "/api/user-stats/scrape",
    responses(
        (status = 200, description = "Successfully scraped user stats"),
    )
)]
pub async fn exec(arc: Data<Dasharr>) -> Result<HttpResponse> {
    scrape_indexers(&arc.pool).await?;

    Ok(HttpResponse::Ok().json("{}"))
}
