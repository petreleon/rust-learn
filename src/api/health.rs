use crate::application::operations::ports::ReadinessDependency;
use crate::application::operations::readiness_check::{
    check_readiness, MissingReadinessDependency, ReadinessStatus,
};
use crate::db::DbPool;
use crate::http::operations::dto::{LivenessResponse, ReadinessResponse};
use crate::infra::ethereum::operations::readiness_check::EthereumReadinessCheck;
use crate::infra::object_storage::operations::readiness_check::S3ReadinessCheck;
use crate::infra::postgres::operations::readiness_check::PostgresReadinessCheck;
use crate::utils::s3_utils::S3State;
use actix_web::{web, HttpResponse, Responder};

pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(LivenessResponse::ok())
}

pub async fn readiness(
    pool: Option<web::Data<DbPool>>,
    s3: Option<web::Data<S3State>>,
) -> impl Responder {
    let mut dependencies = readiness_dependencies(pool, s3);
    let output = check_readiness(&mut dependencies).await;
    let ready = matches!(output.status, ReadinessStatus::Ready);
    let response = ReadinessResponse::from(output);

    if ready {
        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::ServiceUnavailable().json(response)
    }
}

fn readiness_dependencies(
    pool: Option<web::Data<DbPool>>,
    s3: Option<web::Data<S3State>>,
) -> Vec<Box<dyn ReadinessDependency>> {
    let postgres: Box<dyn ReadinessDependency> = match pool {
        Some(pool) => Box::new(PostgresReadinessCheck::new(pool.get_ref().clone())),
        None => Box::new(MissingReadinessDependency::new(
            "postgres",
            "missing database pool",
        )),
    };
    let s3: Box<dyn ReadinessDependency> = match s3 {
        Some(s3) => Box::new(S3ReadinessCheck::new(s3.get_ref().clone())),
        None => Box::new(MissingReadinessDependency::new("s3", "missing S3 state")),
    };

    vec![postgres, s3, Box::new(EthereumReadinessCheck)]
}

pub fn configure_health_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health))
        .route("/ready", web::get().to(readiness));
}

pub fn health_scope() -> actix_web::Scope {
    web::scope("").configure(configure_health_routes)
}
