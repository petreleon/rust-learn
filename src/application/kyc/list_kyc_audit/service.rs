use futures::future::BoxFuture;

use crate::application::kyc::{KycAuditEventOutput, KycAuditQuery, KycError};

pub trait KycAuditUseCase: Send + Sync {
    fn list_submission_audit(
        &self,
        query: KycAuditQuery,
    ) -> BoxFuture<'_, Result<Vec<KycAuditEventOutput>, KycError>>;
}
