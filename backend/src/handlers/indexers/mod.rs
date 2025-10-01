pub mod edit_indexer;
pub mod get_indexer_auth_data;
pub mod get_indexers;
pub mod get_indexers_enriched;
pub mod toggle_indexer;

use actix_web::web::{ServiceConfig, get, put, resource};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        resource("")
            .route(put().to(self::edit_indexer::exec))
            .route(get().to(self::get_indexers::exec)),
    );
    cfg.service(resource("/enriched").route(get().to(self::get_indexers_enriched::exec)));
    cfg.service(resource("/{id}/toggle").route(put().to(self::toggle_indexer::exec)));
    cfg.service(resource("/{id}/auth-data").route(get().to(self::get_indexer_auth_data::exec)));
}
