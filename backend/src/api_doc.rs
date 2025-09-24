use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(title = "arcadia-backend API",),
    paths(crate::handlers::indexers::add_indexer::exec,),
    components(schemas(),)
)]
pub struct ApiDoc;
