use futures::future::BoxFuture;

use crate::application::kyc::{KycError, KycStatusOutput};

pub trait KycStatusUseCase: Send + Sync {
    fn get_my_status(&self, user_id: i32) -> BoxFuture<'_, Result<KycStatusOutput, KycError>>;
}
