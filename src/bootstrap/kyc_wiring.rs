use std::sync::Arc;

use actix_web::web;

use crate::application::kyc::{
    KycAuditUseCase, KycReviewUseCase, KycStatusUseCase, KycSubmissionUseCase,
};
use crate::db::DbPool;
use crate::infra::postgres::kyc::kyc_use_case::PostgresKycUseCase;

#[derive(Clone)]
pub struct KycUseCases {
    pub audit: Arc<dyn KycAuditUseCase>,
    pub review: Arc<dyn KycReviewUseCase>,
    pub status: Arc<dyn KycStatusUseCase>,
    pub submission: Arc<dyn KycSubmissionUseCase>,
}

pub fn build_kyc_use_cases(pool: &DbPool) -> KycUseCases {
    let use_case = Arc::new(PostgresKycUseCase::new(pool.clone()));
    KycUseCases {
        audit: use_case.clone(),
        review: use_case.clone(),
        status: use_case.clone(),
        submission: use_case,
    }
}

pub fn configure_kyc_app_data(cfg: &mut web::ServiceConfig, use_cases: &KycUseCases) {
    cfg.app_data(web::Data::new(use_cases.audit.clone()))
        .app_data(web::Data::new(use_cases.review.clone()))
        .app_data(web::Data::new(use_cases.status.clone()))
        .app_data(web::Data::new(use_cases.submission.clone()));
}
