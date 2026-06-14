use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::kyc::{KycAuditEventOutput, KycError, KycStore, KycSubmissionOutput};
use crate::domain::kyc::submission::{NormalizedKycDecision, NormalizedKycSubmission};
use crate::infra::postgres::kyc::kyc_permission_queries;
use crate::infra::postgres::kyc::kyc_read_queries;
use crate::infra::postgres::kyc::kyc_transactions::{create_submission, decide_submission};

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
        async move { kyc_read_queries::get_user_kyc_verified(self.conn, user_id).await }.boxed()
    }

    fn latest_submission_for_user(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<Option<KycSubmissionOutput>, KycError>> {
        async move { kyc_read_queries::latest_submission_for_user(self.conn, user_id).await }
            .boxed()
    }

    fn create_submission(
        &mut self,
        submission: NormalizedKycSubmission,
    ) -> BoxFuture<'_, Result<KycSubmissionOutput, KycError>> {
        async move { create_submission(self.conn, submission).await }.boxed()
    }

    fn can_review_kyc(&mut self, user_id: i32) -> BoxFuture<'_, Result<bool, KycError>> {
        async move { kyc_permission_queries::can_review_kyc(self.conn, user_id).await }.boxed()
    }

    fn list_review_queue(&mut self) -> BoxFuture<'_, Result<Vec<KycSubmissionOutput>, KycError>> {
        async move { kyc_read_queries::list_review_queue(self.conn).await }.boxed()
    }

    fn find_submission(
        &mut self,
        submission_id: i64,
    ) -> BoxFuture<'_, Result<KycSubmissionOutput, KycError>> {
        async move { kyc_read_queries::find_submission(self.conn, submission_id).await }.boxed()
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
        async move { kyc_read_queries::list_submission_audit(self.conn, submission_id).await }
            .boxed()
    }
}
