use utoipa::OpenApi;

use crate::handlers::user_stats::get_user_stats::GetUserStatsQuery;

#[derive(OpenApi)]
#[openapi(
    info(title = "dasharr-backend API",),
    paths(
        crate::handlers::indexers::edit_indexer::exec,
        crate::handlers::indexers::get_indexers::exec,
        crate::handlers::indexers::get_indexer_auth_data::exec,
        crate::handlers::indexers::get_indexers_enriched::exec,
        crate::handlers::indexers::toggle_indexer::exec,
        crate::handlers::user_stats::scrape_user_stats::exec,
        crate::handlers::user_stats::get_user_stats::exec,
        crate::handlers::user_stats::get_user_stats_prometheus::exec,
    ),
    components(schemas(GetUserStatsQuery),)
)]
pub struct ApiDoc;
