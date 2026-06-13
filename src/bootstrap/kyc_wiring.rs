use std::sync::Arc;

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
