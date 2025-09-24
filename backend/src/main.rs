use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware, web::Data};
use dasharr::{Dasharr, api_doc::ApiDoc, connection_pool::ConnectionPool, env::Env, routes::init};
use envconfig::Envconfig;
use std::{env, sync::Arc};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
// use utoipa::OpenApi;
// use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    if env::var("ENV").unwrap_or("".to_string()) != "Docker" {
        dotenvy::from_filename(".env").expect("error while loading env from .env file");
    }
    let env = Env::init_from_env().unwrap();
    let pool = Arc::new(
        ConnectionPool::try_new(&env.database_url)
            .await
            .expect("db connection couldn't be established"),
    );

    let arc = Data::new(Dasharr::new(Arc::clone(&pool), env));

    let server_url = "127.0.0.1:8080".to_string();
    println!("Server running at http://{server_url}");

    let server = HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(arc.clone())
            .configure(init) // Initialize routes
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/swagger-json/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(server_url)?
    .run();

    server.await
}
