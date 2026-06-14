use serde::Serialize;

use crate::application::operations::readiness_check::{
    DependencyCheckOutput, DependencyStatus, ReadinessOutput, ReadinessStatus,
};

#[derive(Debug, Clone, Serialize)]
pub struct LivenessResponse {
    pub status: &'static str,
}

impl LivenessResponse {
    pub fn ok() -> Self {
        Self { status: "ok" }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ReadinessResponse {
    pub status: &'static str,
    pub checks: Vec<DependencyCheckResponse>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DependencyCheckResponse {
    pub name: &'static str,
    pub status: &'static str,
    pub message: Option<String>,
}

impl From<ReadinessOutput> for ReadinessResponse {
    fn from(output: ReadinessOutput) -> Self {
        Self {
            status: match output.status {
                ReadinessStatus::Ready => "ready",
                ReadinessStatus::NotReady => "not_ready",
            },
            checks: output
                .checks
                .into_iter()
                .map(DependencyCheckResponse::from)
                .collect(),
        }
    }
}

impl From<DependencyCheckOutput> for DependencyCheckResponse {
    fn from(check: DependencyCheckOutput) -> Self {
        Self {
            name: check.name,
            status: match check.status {
                DependencyStatus::Ok => "ok",
                DependencyStatus::Failed => "failed",
            },
            message: check.message,
        }
    }
}
