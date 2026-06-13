use std::sync::Arc;

use actix_web::web;
use futures::future::{ready, BoxFuture, FutureExt};
use rust_learn::application::kyc::{
    KycAuditEventOutput, KycAuditQuery, KycAuditUseCase, KycDecisionCommand, KycError,
    KycReviewQueueOutput, KycReviewUseCase, KycStatusOutput, KycStatusUseCase, KycSubmissionOutput,
    KycSubmissionUseCase, SubmitKycCommand,
};

struct RouteOnlyKycUseCase;

pub fn kyc_status_data() -> web::Data<Arc<dyn KycStatusUseCase>> {
    web::Data::new(Arc::new(RouteOnlyKycUseCase))
}

pub fn kyc_submission_data() -> web::Data<Arc<dyn KycSubmissionUseCase>> {
    web::Data::new(Arc::new(RouteOnlyKycUseCase))
}

pub fn kyc_review_data() -> web::Data<Arc<dyn KycReviewUseCase>> {
    web::Data::new(Arc::new(RouteOnlyKycUseCase))
}

pub fn kyc_audit_data() -> web::Data<Arc<dyn KycAuditUseCase>> {
    web::Data::new(Arc::new(RouteOnlyKycUseCase))
}

impl KycStatusUseCase for RouteOnlyKycUseCase {
    fn get_my_status(&self, _user_id: i32) -> BoxFuture<'_, Result<KycStatusOutput, KycError>> {
        ready(Ok(KycStatusOutput {
            next_action: "submit".to_string(),
            submission: None,
            user_kyc_verified: false,
        }))
        .boxed()
    }
}

impl KycSubmissionUseCase for RouteOnlyKycUseCase {
    fn submit_my_kyc(
        &self,
        _command: SubmitKycCommand,
    ) -> BoxFuture<'_, Result<KycStatusOutput, KycError>> {
        ready(Ok(KycStatusOutput {
            next_action: "wait_for_review".to_string(),
            submission: Some(submission(34)),
            user_kyc_verified: false,
        }))
        .boxed()
    }
}

impl KycReviewUseCase for RouteOnlyKycUseCase {
    fn list_review_queue(
        &self,
        _reviewer_user_id: i32,
    ) -> BoxFuture<'_, Result<KycReviewQueueOutput, KycError>> {
        ready(Ok(KycReviewQueueOutput {
            submissions: vec![submission(34)],
        }))
        .boxed()
    }

    fn decide_submission(
        &self,
        command: KycDecisionCommand,
    ) -> BoxFuture<'_, Result<KycSubmissionOutput, KycError>> {
        ready(Ok(submission(command.submission_id))).boxed()
    }
}

impl KycAuditUseCase for RouteOnlyKycUseCase {
    fn list_submission_audit(
        &self,
        _query: KycAuditQuery,
    ) -> BoxFuture<'_, Result<Vec<KycAuditEventOutput>, KycError>> {
        ready(Ok(Vec::new())).boxed()
    }
}

fn submission(id: i64) -> KycSubmissionOutput {
    let now = chrono::Utc::now();
    KycSubmissionOutput {
        country_code: "US".to_string(),
        created_at: now,
        document_last4: Some("1234".to_string()),
        document_type: "passport".to_string(),
        evidence_reference: None,
        id,
        legal_name: "Route Smoke".to_string(),
        provider_reference: None,
        rejection_reason: None,
        reviewed_at: None,
        reviewer_user_id: None,
        status: "submitted".to_string(),
        submitted_at: now,
        updated_at: now,
        user_id: 12,
    }
}
