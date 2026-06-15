use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, FromRequest, HttpMessage, HttpRequest};
use futures::future::{ready, Ready};

use crate::domain::identity::UserJWT;

#[derive(Clone)]
pub struct AuthUser(pub UserJWT);

#[derive(Clone, Copy, Debug)]
pub struct AuthUserId(pub i32);

impl AuthUser {
    pub fn user_id(&self) -> i32 {
        self.0.user_id
    }

    pub fn into_inner(self) -> UserJWT {
        self.0
    }
}

impl AuthUserId {
    pub fn into_inner(self) -> i32 {
        self.0
    }
}

impl FromRequest for AuthUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(auth_user_from_request(req))
    }
}

impl FromRequest for AuthUserId {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(auth_user_id_from_request(req))
    }
}

fn auth_user_id_from_request(req: &HttpRequest) -> Result<AuthUserId, Error> {
    if let Some(user_jwt) = req.extensions().get::<UserJWT>() {
        return Ok(AuthUserId(user_jwt.user_id));
    }

    let Some(auth_header) = req.headers().get("Authorization") else {
        return Err(ErrorUnauthorized("Missing Authorization header"));
    };
    let Ok(auth_str) = auth_header.to_str() else {
        return Err(ErrorUnauthorized("Invalid Authorization header format"));
    };
    let Some(_token) = auth_str.strip_prefix("Bearer ") else {
        return Err(ErrorUnauthorized("Invalid Authorization header format"));
    };

    Err(ErrorUnauthorized("Invalid token"))
}

fn auth_user_from_request(req: &HttpRequest) -> Result<AuthUser, Error> {
    if let Some(user_jwt) = req.extensions().get::<UserJWT>() {
        return Ok(AuthUser(user_jwt.clone()));
    }

    let Some(auth_header) = req.headers().get("Authorization") else {
        return Err(ErrorUnauthorized("Missing Authorization header"));
    };
    let Ok(auth_str) = auth_header.to_str() else {
        return Err(ErrorUnauthorized("Invalid Authorization header format"));
    };
    let Some(_token) = auth_str.strip_prefix("Bearer ") else {
        return Err(ErrorUnauthorized("Invalid Authorization header format"));
    };

    Err(ErrorUnauthorized("Invalid token"))
}

#[cfg(test)]
mod tests {
    use super::{auth_user_from_request, auth_user_id_from_request};
    use crate::domain::identity::UserJWT;
    use actix_web::{body::to_bytes, http::StatusCode, test as actix_test, HttpMessage};

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

    #[actix_web::test]
    async fn rejects_missing_authorization_header() {
        let req = actix_test::TestRequest::default().to_http_request();

        let error = match auth_user_from_request(&req) {
            Ok(user) => panic!("expected unauthorized error, got user {}", user.user_id()),
            Err(error) => error,
        };

        assert_eq!(
            error.as_response_error().status_code(),
            StatusCode::UNAUTHORIZED
        );
        let body = to_bytes(error.as_response_error().error_response().into_body())
            .await
            .unwrap();
        assert_eq!(body.as_ref(), b"Missing Authorization header");
    }

    #[actix_web::test]
    async fn rejects_non_bearer_authorization_header() {
        let req = actix_test::TestRequest::default()
            .insert_header(("Authorization", "Token abc123"))
            .to_http_request();

        let error = match auth_user_from_request(&req) {
            Ok(user) => panic!("expected unauthorized error, got user {}", user.user_id()),
            Err(error) => error,
        };

        assert_eq!(
            error.as_response_error().status_code(),
            StatusCode::UNAUTHORIZED
        );
        let body = to_bytes(error.as_response_error().error_response().into_body())
            .await
            .unwrap();
        assert_eq!(body.as_ref(), b"Invalid Authorization header format");
    }

    #[test]
    fn user_id_extractor_reads_user_from_request_extensions() {
        let req = actix_test::TestRequest::default().to_http_request();
        req.extensions_mut().insert(UserJWT {
            user_id: 99,
            exp: 1000,
        });

        let user_id = auth_user_id_from_request(&req).unwrap();

        assert_eq!(user_id.into_inner(), 99);
    }

    #[test]
    fn user_id_extractor_rejects_missing_authorization_header() {
        let req = actix_test::TestRequest::default().to_http_request();

        let error = auth_user_id_from_request(&req).unwrap_err();

        assert_eq!(
            error.as_response_error().status_code(),
            StatusCode::UNAUTHORIZED
        );
    }
}
