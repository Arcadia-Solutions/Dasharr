pub mod edit_indexer;
pub mod get_indexers;
pub mod toggle_indexer;

use actix_web::web::{ServiceConfig, get, put, resource};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(put().to(self::edit_indexer::exec))
            .route(get().to(self::get_indexers::exec)),
    );
    cfg.service(resource("/{id}/toggle").route(put().to(self::toggle_indexer::exec)));
}
