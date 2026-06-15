use std::sync::Arc;

use actix_web::{http::StatusCode, web};

use crate::application::operations::ports::ReadinessDependency;
use crate::application::operations::readiness_check::{
    check_readiness, MissingReadinessDependency, ReadinessStatus, ReadinessUseCase,
};
use crate::http::operations::dto::{LivenessResponse, ReadinessResponse};

pub async fn health() -> web::Json<LivenessResponse> {
    web::Json(LivenessResponse::ok())
}

pub async fn readiness(
    use_case: Option<web::Data<Arc<dyn ReadinessUseCase>>>,
) -> (web::Json<ReadinessResponse>, StatusCode) {
    let output = match use_case {
        Some(use_case) => use_case.check().await,
        None => missing_readiness().await,
    };
    let status = readiness_status_code(output.status);
    let response = ReadinessResponse::from(output);

    (web::Json(response), status)
}

fn readiness_status_code(status: ReadinessStatus) -> StatusCode {
    match status {
        ReadinessStatus::Ready => StatusCode::OK,
        ReadinessStatus::NotReady => StatusCode::SERVICE_UNAVAILABLE,
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
