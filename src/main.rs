pub mod api;
pub mod application;
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

use crate::config::db_setup::version_updater;
use crate::utils::s3_utils::S3State;
use actix_web::rt::time::timeout;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use std::time::Duration;

const ETH_STARTUP_DEPLOY_TIMEOUT: Duration = Duration::from_secs(30);

#[get("/")]
async fn api_index() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "service": "rust-learn-api",
        "health": "/health",
        "readiness": "/ready",
        "api": "/api"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    crate::utils::logging::init_logging("api");

    let pool = match db::try_establish_connection() {
        Ok(pool) => pool,
        Err(error) => {
            log::error!("event=db_pool_init_failed error={}", error);
            return Err(std::io::Error::other(error));
        }
    };
    // Initialize S3 client state and put into app data
    let s3_state = match S3State::new_from_env().await {
        Ok(s) => s,
        Err(e) => {
            log::error!("event=s3_init_failed error={:?}", e);
            return Err(std::io::Error::other("S3 init failed"));
        }
    };
    // Initialize notifications state (DB-backed using the pool)
    let notifications_state = crate::utils::notifications::NotificationsState::new(pool.clone());
    {
        let mut conn = match pool.get().await {
            Ok(conn) => conn,
            Err(error) => {
                log::error!("event=db_connection_failed phase=startup error={:?}", error);
                return Err(std::io::Error::other(format!(
                    "Failed to get DB connection from pool: {error}"
                )));
            }
        };
        if let Err(error) = version_updater(&mut conn).await {
            log::error!("event=db_version_update_failed error={:?}", error);
            return Err(std::io::Error::other(format!(
                "Failed to update database version: {error}"
            )));
        }

        // Ensure LearnToken and wallet transfer helper contracts are deployed
        // idempotently and persisted for API/worker use.
        match timeout(
            ETH_STARTUP_DEPLOY_TIMEOUT,
            crate::utils::eth_utils::deploy_all_startup(&mut conn, "LearnToken", "LRN", 18),
        )
        .await
        {
            Ok(Ok((token_addr, presigner_addr, importer_addr))) => {
                let presigner_address = presigner_addr
                    .map(|addr| format!("{:#x}", addr))
                    .unwrap_or_else(|| "none".to_string());
                let importer_address = importer_addr
                    .map(|addr| format!("{:#x}", addr))
                    .unwrap_or_else(|| "none".to_string());
                log::info!(
                    "event=eth_startup_deploy_ready token_address={:#x} presigner_address={} importer_address={}",
                    token_addr,
                    presigner_address,
                    importer_address
                );
            }
            Ok(Err(err)) => log::error!(
                "event=eth_startup_deploy_failed contract_scope=wallet_transfer error={:?}",
                err
            ),
            Err(_) => log::error!(
                "event=eth_startup_deploy_timed_out contract_scope=wallet_transfer timeout_seconds={}",
                ETH_STARTUP_DEPLOY_TIMEOUT.as_secs()
            ),
        }
    }
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Use the created pool
            .app_data(web::Data::new(s3_state.clone())) // S3 client shared state
            .app_data(web::Data::new(notifications_state.clone())) // Notifications shared state
            .route(
                "/.well-known/jwks.json",
                web::get().to(api::authentication::jwks),
            )
            .configure(api::health::configure_health_routes)
            .service(api::api_scope())
            .service(api_index)
    })
    .bind("0.0.0.0:8080")? // Update the bind address if necessary
    .run()
    .await
}
