use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(title = "arcadia-backend API",),
    paths(
        crate::handlers::indexers::edit_indexer::exec,
        crate::handlers::indexers::get_indexers::exec,
    ),
    components(schemas(),)
)]
pub struct ApiDoc;
