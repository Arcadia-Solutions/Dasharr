#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("could not create indexer")]
    CouldNotCreateIndexer(#[source] sqlx::Error),
    #[error("could not toggle indexer")]
    CouldNotToggleIndexer(#[source] sqlx::Error),
    #[error("could not get indexers")]
    CouldNotGetIndexers(#[source] sqlx::Error),
    #[error("could not insert stats: {0}")]
    CouldNotInsertStats(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl actix_web::ResponseError for Error {
    #[inline]
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        // match self {
        //     // 500 Internal Server Error
        //     _ => StatusCode::INTERNAL_SERVER_ERROR,
        // }
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        log::error!("The request generated this error: {self}");
        actix_web::HttpResponse::build(self.status_code()).json(serde_json::json!({
            "error": format!("{self}"),
        }))
    }
}
