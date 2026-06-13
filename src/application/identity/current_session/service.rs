use futures::future::BoxFuture;

use crate::application::identity::current_session::{CurrentSessionError, CurrentSessionOutput};

pub trait CurrentSessionUseCase: Send + Sync {
    fn get_current_session(
        &self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<CurrentSessionOutput, CurrentSessionError>>;
}
