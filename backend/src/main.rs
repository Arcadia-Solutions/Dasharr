use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware, web::Data};
use dasharr::{
    Dasharr, api_doc::ApiDoc, connection_pool::ConnectionPool, env::Env, routes::init,
    scheduler::run_periodic_tasks,
};
use envconfig::Envconfig;
use std::borrow::Borrow;
use std::ops::Deref;
use std::{env, sync::Arc};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    if env::var("ENV").unwrap_or("".to_string()) != "Docker" {
        dotenvy::from_filename(".env").expect("error while loading env from .env file");
    }
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));
    let env = Env::init_from_env().unwrap();
    let pool = Arc::new(
        ConnectionPool::try_new(&env.database_url)
            .await
            .expect("db connection couldn't be established"),
    );

    sqlx::migrate!("./migrations")
        .run(pool.deref().borrow())
        .await
        .expect("Failed to run database migrations!");

    let arc = Data::new(Dasharr::new(Arc::clone(&pool), env.clone()));

    let web_server_port = env::var("WEB_SERVER_PORT").expect("WEB_SERVER_PORT must be set");
    let server_url = format!("127.0.0.1:{web_server_port}").to_string();
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

    let arc_2 = Arc::new(Dasharr::new(Arc::clone(&pool), env.clone()));
    if let Err(e) = run_periodic_tasks(arc_2).await {
        eprintln!("Error running cron tasks: {e:?}");
    }

    server.await
}
