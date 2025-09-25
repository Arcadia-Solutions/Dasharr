use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(title = "arcadia-backend API",),
    paths(
        crate::handlers::indexers::edit_indexer::exec,
        crate::handlers::indexers::get_indexers::exec,
        crate::handlers::user_stats::scrape_user_stats::exec,
        crate::handlers::user_stats::get_user_stats::exec,
    ),
    components(schemas(),)
)]
pub struct ApiDoc;
