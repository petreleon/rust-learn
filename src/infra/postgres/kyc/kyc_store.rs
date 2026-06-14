use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::kyc::{KycAuditEventOutput, KycError, KycStore, KycSubmissionOutput};
use crate::config::constants::permissions::Permissions;
use crate::db::schema::{kyc_audit_events, kyc_submissions, users};
use crate::domain::kyc::submission::{
    NormalizedKycDecision, NormalizedKycSubmission, KYC_STATUS_SUBMITTED, KYC_STATUS_UNDER_REVIEW,
};
use crate::infra::postgres::kyc::kyc_mappers::map_error;
use crate::infra::postgres::kyc::kyc_permissions::user_has_platform_permission;
use crate::infra::postgres::kyc::kyc_transactions::{create_submission, decide_submission};
use crate::models::kyc_audit_event::KycAuditEvent;
use crate::models::kyc_submission::KycSubmission;

pub struct PostgresKycStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresKycStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl KycStore for PostgresKycStore<'_> {
    fn get_user_kyc_verified(&mut self, user_id: i32) -> BoxFuture<'_, Result<bool, KycError>> {
        async move {
            users::table
                .find(user_id)
                .select(users::kyc_verified)
                .first::<bool>(self.conn)
                .await
                .map_err(map_error)
        }
        .boxed()
    }

    fn latest_submission_for_user(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<Option<KycSubmissionOutput>, KycError>> {
        async move {
            kyc_submissions::table
                .filter(kyc_submissions::user_id.eq(user_id))
                .order(kyc_submissions::updated_at.desc())
                .then_order_by(kyc_submissions::id.desc())
                .first::<KycSubmission>(self.conn)
                .await
                .optional()
                .map(|submission| submission.map(Into::into))
                .map_err(map_error)
        }
        .boxed()
    }

    fn create_submission(
        &mut self,
        submission: NormalizedKycSubmission,
    ) -> BoxFuture<'_, Result<KycSubmissionOutput, KycError>> {
        async move { create_submission(self.conn, submission).await }.boxed()
    }

    fn can_review_kyc(&mut self, user_id: i32) -> BoxFuture<'_, Result<bool, KycError>> {
        async move {
            user_has_platform_permission(
                self.conn,
                user_id,
                &Permissions::REVIEW_KYC_SUBMISSIONS.to_string(),
            )
            .await
            .map_err(map_error)
        }
        .boxed()
    }

    fn list_review_queue(&mut self) -> BoxFuture<'_, Result<Vec<KycSubmissionOutput>, KycError>> {
        async move {
            kyc_submissions::table
                .filter(
                    kyc_submissions::status.eq_any([KYC_STATUS_SUBMITTED, KYC_STATUS_UNDER_REVIEW]),
                )
                .order(kyc_submissions::submitted_at.asc())
                .then_order_by(kyc_submissions::id.asc())
                .limit(50)
                .load::<KycSubmission>(self.conn)
                .await
                .map(|items| items.into_iter().map(Into::into).collect())
                .map_err(map_error)
        }
        .boxed()
    }

    fn find_submission(
        &mut self,
        submission_id: i64,
    ) -> BoxFuture<'_, Result<KycSubmissionOutput, KycError>> {
        async move {
            kyc_submissions::table
                .find(submission_id)
                .first::<KycSubmission>(self.conn)
                .await
                .map(Into::into)
                .map_err(map_error)
        }
        .boxed()
    }

    fn decide_submission(
        &mut self,
        submission_id: i64,
        reviewer_user_id: i32,
        from_status: String,
        decision: NormalizedKycDecision,
    ) -> BoxFuture<'_, Result<KycSubmissionOutput, KycError>> {
        async move {
            decide_submission(
                self.conn,
                submission_id,
                reviewer_user_id,
                from_status,
                decision,
            )
            .await
        }
        .boxed()
    }

    fn list_submission_audit(
        &mut self,
        submission_id: i64,
    ) -> BoxFuture<'_, Result<Vec<KycAuditEventOutput>, KycError>> {
        async move {
            kyc_audit_events::table
                .filter(kyc_audit_events::submission_id.eq(submission_id))
                .order(kyc_audit_events::created_at.asc())
                .then_order_by(kyc_audit_events::id.asc())
                .load::<KycAuditEvent>(self.conn)
                .await
                .map(|items| items.into_iter().map(Into::into).collect())
                .map_err(map_error)
        }
        .boxed()
    }
}
