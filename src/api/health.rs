use crate::db::DbPool;
use crate::utils::eth::provider::try_get_provider;
use crate::utils::s3_utils::S3State;
use actix_web::rt::time::timeout;
use actix_web::{web, HttpResponse, Responder};
use anyhow::{anyhow, Result};
use diesel::sql_types::Integer;
use diesel_async::RunQueryDsl;
use ethers::providers::Middleware;
use serde::Serialize;
use std::future::Future;
use std::time::Duration;

const CHECK_TIMEOUT: Duration = Duration::from_secs(3);

#[derive(Serialize)]
struct LivenessResponse {
    status: &'static str,
}

#[derive(Serialize)]
struct ReadinessResponse {
    status: &'static str,
    checks: Vec<DependencyCheck>,
}

#[derive(Serialize)]
struct DependencyCheck {
    name: &'static str,
    status: &'static str,
    message: Option<String>,
}

pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(LivenessResponse { status: "ok" })
}

pub async fn readiness(
    pool: Option<web::Data<DbPool>>,
    s3: Option<web::Data<S3State>>,
) -> impl Responder {
    let db_check = async move {
        match pool {
            Some(pool) => check_db(pool).await,
            None => failed("postgres", "missing database pool"),
        }
    };
    let s3_check = async move {
        match s3 {
            Some(s3) => check_s3(s3).await,
            None => failed("s3", "missing S3 state"),
        }
    };
    let eth_check = check_ethereum();

    let (db, s3, ethereum) = tokio::join!(db_check, s3_check, eth_check);
    let checks = vec![db, s3, ethereum];

    let ready = checks.iter().all(|check| check.status == "ok");
    let response = ReadinessResponse {
        status: if ready { "ready" } else { "not_ready" },
        checks,
    };

    if ready {
        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::ServiceUnavailable().json(response)
    }
}

async fn check_db(pool: web::Data<DbPool>) -> DependencyCheck {
    with_timeout("postgres", async move {
        let mut conn = pool
            .get()
            .await
            .map_err(|err| anyhow!("database pool checkout failed: {err}"))?;
        let _: i32 = diesel::select(diesel::dsl::sql::<Integer>("1"))
            .get_result(&mut conn)
            .await?;
        Ok(())
    })
    .await
}

async fn check_s3(s3: web::Data<S3State>) -> DependencyCheck {
    with_timeout("s3", async move { s3.health_check().await }).await
}

async fn check_ethereum() -> DependencyCheck {
    with_timeout("ethereum", async move {
        let provider = try_get_provider().map_err(anyhow::Error::msg)?;
        provider.get_chainid().await?;
        Ok(())
    })
    .await
}

async fn with_timeout<F>(name: &'static str, check: F) -> DependencyCheck
where
    F: Future<Output = Result<()>>,
{
    match timeout(CHECK_TIMEOUT, check).await {
        Ok(Ok(())) => DependencyCheck {
            name,
            status: "ok",
            message: None,
        },
        Ok(Err(err)) => failed(name, err.to_string()),
        Err(_) => failed(name, "timed out"),
    }
}

fn failed(name: &'static str, message: impl Into<String>) -> DependencyCheck {
    DependencyCheck {
        name,
        status: "failed",
        message: Some(message.into()),
    }
}

pub fn configure_health_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health))
        .route("/ready", web::get().to(readiness));
}

pub fn health_scope() -> actix_web::Scope {
    web::scope("").configure(configure_health_routes)
}
