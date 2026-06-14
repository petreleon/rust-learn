use futures::future::BoxFuture;

use crate::application::kyc::{KycError, KycStatusOutput, SubmitKycCommand};

pub trait KycSubmissionUseCase: Send + Sync {
    fn submit_my_kyc(
        &self,
        command: SubmitKycCommand,
    ) -> BoxFuture<'_, Result<KycStatusOutput, KycError>>;
}
