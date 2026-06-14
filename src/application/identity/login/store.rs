use futures::future::BoxFuture;

use super::{LoginAuthentication, LoginError};

pub trait LoginStore {
    fn find_password_auth(
        &mut self,
        email: String,
    ) -> BoxFuture<'_, Result<Option<LoginAuthentication>, LoginError>>;
}
