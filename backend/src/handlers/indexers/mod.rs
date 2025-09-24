pub mod add_indexer;

use actix_web::web::{ServiceConfig, post, resource};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(resource("").route(post().to(self::add_indexer::exec)));
}
