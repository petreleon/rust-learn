pub mod api;
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
pub mod services;
pub mod shared;
pub mod utils;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    crate::utils::logging::init_logging("api");

    let app_state = bootstrap::startup::initialize_app_state().await?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.pool.clone()))
            .app_data(web::Data::new(app_state.s3.clone()))
            .app_data(web::Data::new(app_state.notifications.clone()))
            .configure(bootstrap::routes::configure_routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
