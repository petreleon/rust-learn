use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::operations::ports::ReadinessDependency;
use crate::application::operations::readiness_check::{
    check_readiness, MissingReadinessDependency, ReadinessStatus, ReadinessUseCase,
};
use crate::http::operations::dto::{LivenessResponse, ReadinessResponse};

pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(LivenessResponse::ok())
}

pub async fn readiness(use_case: Option<web::Data<Arc<dyn ReadinessUseCase>>>) -> impl Responder {
    let output = match use_case {
        Some(use_case) => use_case.check().await,
        None => missing_readiness().await,
    };
    let ready = matches!(output.status, ReadinessStatus::Ready);
    let response = ReadinessResponse::from(output);

    if ready {
        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::ServiceUnavailable().json(response)
    }
}

async fn missing_readiness() -> crate::application::operations::readiness_check::ReadinessOutput {
    let mut dependencies: Vec<Box<dyn ReadinessDependency>> = vec![
        Box::new(MissingReadinessDependency::new(
            "postgres",
            "missing database pool",
        )),
        Box::new(MissingReadinessDependency::new("s3", "missing S3 state")),
        Box::new(MissingReadinessDependency::new(
            "ethereum",
            "missing readiness use case",
        )),
    ];

    check_readiness(&mut dependencies).await
}
