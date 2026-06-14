pub mod application;
pub mod bootstrap;
pub mod config;
pub mod db;
pub mod domain;
pub mod http;
pub mod infra;
pub mod middlewares;
mod models;
pub mod repositories;
pub mod shared;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    crate::bootstrap::logging::init_logging("api");

    let app_state = bootstrap::startup::initialize_app_state().await?;

    HttpServer::new(move || {
        App::new()
            .configure(|cfg| bootstrap::app_data::configure_app_data(cfg, &app_state))
            .configure(bootstrap::routes::configure_routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
