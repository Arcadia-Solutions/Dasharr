use actix_web::web::{self, scope};
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::{
    handlers::indexers::config as IndexersConfig, handlers::user_stats::config as UserStatsConfig,
    middleware::authenticate_user,
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .wrap(HttpAuthentication::with_fn(authenticate_user))
            .service(scope("/indexers").configure(IndexersConfig))
            .service(scope("/user-stats").configure(UserStatsConfig)),
    );
}
