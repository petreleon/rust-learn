use actix_web::dev::Payload;
use actix_web::{Error, FromRequest, HttpMessage, HttpRequest};
use futures::future::{ready, Ready};

use crate::domain::identity::UserJWT;
use crate::http::errors::ApiError;

#[derive(Clone)]
pub struct AuthUser(pub UserJWT);

impl AuthUser {
    pub fn user_id(&self) -> i32 {
        self.0.user_id
    }

    pub fn into_inner(self) -> UserJWT {
        self.0
    }
}

impl FromRequest for AuthUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(auth_user_from_request(req).map_err(Into::into))
    }
}

fn auth_user_from_request(req: &HttpRequest) -> Result<AuthUser, ApiError> {
    if let Some(user_jwt) = req.extensions().get::<UserJWT>() {
        return Ok(AuthUser(user_jwt.clone()));
    }

    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| ApiError::unauthorized("Missing Authorization header"))?;
    let auth_str = auth_header
        .to_str()
        .map_err(|_| ApiError::unauthorized("Invalid Authorization header format"))?;
    let _token = auth_str
        .strip_prefix("Bearer ")
        .ok_or_else(|| ApiError::unauthorized("Invalid Authorization header format"))?;

    Err(ApiError::unauthorized("Invalid token"))
}

#[cfg(test)]
mod tests {
    use super::auth_user_from_request;
    use crate::domain::identity::UserJWT;
    use actix_web::{http::StatusCode, test as actix_test, HttpMessage, ResponseError};

    #[test]
    fn reads_user_from_request_extensions() {
        let req = actix_test::TestRequest::default().to_http_request();
        req.extensions_mut().insert(UserJWT {
            user_id: 42,
            exp: 1000,
        });

        let user = auth_user_from_request(&req).unwrap();

        assert_eq!(user.user_id(), 42);
        assert_eq!(user.into_inner().exp, 1000);
    }

    #[test]
    fn rejects_missing_authorization_header() {
        let req = actix_test::TestRequest::default().to_http_request();

        let error = match auth_user_from_request(&req) {
            Ok(user) => panic!("expected unauthorized error, got user {}", user.user_id()),
            Err(error) => error,
        };

        assert_eq!(error.status_code(), StatusCode::UNAUTHORIZED);
        assert_eq!(error.code(), "unauthorized");
    }

    #[test]
    fn rejects_non_bearer_authorization_header() {
        let req = actix_test::TestRequest::default()
            .insert_header(("Authorization", "Token abc123"))
            .to_http_request();

        let error = match auth_user_from_request(&req) {
            Ok(user) => panic!("expected unauthorized error, got user {}", user.user_id()),
            Err(error) => error,
        };

        assert_eq!(error.status_code(), StatusCode::UNAUTHORIZED);
    }
}
