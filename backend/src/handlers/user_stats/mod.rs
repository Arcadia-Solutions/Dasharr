pub mod scrape_user_stats;

use actix_web::web::{ServiceConfig, get, resource};

pub fn config(cfg: &mut ServiceConfig) {
    // cfg.service(resource("").route(get().to(self::get_indexers::exec)));
    cfg.service(resource("/scrape").route(get().to(self::scrape_user_stats::exec)));
}
