use diesel_async::pooled_connection::deadpool::Object;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::kyc::{
    self, KycAuditEventOutput, KycAuditQuery, KycAuditUseCase, KycDecisionCommand, KycError,
    KycReviewQueueOutput, KycReviewUseCase, KycStatusOutput, KycStatusUseCase, KycSubmissionOutput,
    KycSubmissionUseCase, SubmitKycCommand,
};
use crate::infra::postgres::kyc::kyc_store::PostgresKycStore;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct PostgresKycUseCase {
    pool: DbPool,
}

impl PostgresKycUseCase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl KycStatusUseCase for PostgresKycUseCase {
    fn get_my_status(&self, user_id: i32) -> BoxFuture<'_, Result<KycStatusOutput, KycError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresKycStore::new(&mut conn);
            kyc::get_kyc_status::get_my_status(&mut store, user_id).await
        }
        .boxed()
    }
}

impl KycSubmissionUseCase for PostgresKycUseCase {
    fn submit_my_kyc(
        &self,
        command: SubmitKycCommand,
    ) -> BoxFuture<'_, Result<KycStatusOutput, KycError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresKycStore::new(&mut conn);
            kyc::submit_kyc::submit_my_kyc(&mut store, command).await
        }
        .boxed()
    }
}

impl KycReviewUseCase for PostgresKycUseCase {
    fn list_review_queue(
        &self,
        reviewer_user_id: i32,
    ) -> BoxFuture<'_, Result<KycReviewQueueOutput, KycError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresKycStore::new(&mut conn);
            kyc::review_kyc::list_review_queue(&mut store, reviewer_user_id).await
        }
        .boxed()
    }

    fn decide_submission(
        &self,
        command: KycDecisionCommand,
    ) -> BoxFuture<'_, Result<KycSubmissionOutput, KycError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresKycStore::new(&mut conn);
            kyc::review_kyc::decide_submission(&mut store, command).await
        }
        .boxed()
    }
}

impl KycAuditUseCase for PostgresKycUseCase {
    fn list_submission_audit(
        &self,
        query: KycAuditQuery,
    ) -> BoxFuture<'_, Result<Vec<KycAuditEventOutput>, KycError>> {
        async move {
            let mut conn = self.connection().await?;
            let mut store = PostgresKycStore::new(&mut conn);
            kyc::list_kyc_audit::list_submission_audit(&mut store, query).await
        }
        .boxed()
    }
}

impl PostgresKycUseCase {
    async fn connection(&self) -> Result<Object<AsyncPgConnection>, KycError> {
        self.pool
            .get()
            .await
            .map_err(|error| KycError::Connection(error.to_string()))
    }
}
