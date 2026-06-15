use actix_web::{HttpMessage, HttpRequest, HttpResponse};

use crate::domain::identity::UserJWT;

pub(crate) fn authenticated_user(req: &HttpRequest) -> Result<UserJWT, HttpResponse> {
    if let Some(user_jwt) = req.extensions().get::<UserJWT>() {
        return Ok(user_jwt.clone());
    }

    let Some(auth_header) = req.headers().get("Authorization") else {
        return Err(HttpResponse::Unauthorized().body("Missing Authorization header"));
    };

    let Ok(auth_str) = auth_header.to_str() else {
        return Err(HttpResponse::Unauthorized().body("Invalid Authorization header format"));
    };

    let Some(_token) = auth_str.strip_prefix("Bearer ") else {
        return Err(HttpResponse::Unauthorized().body("Invalid Authorization header format"));
    };

    Err(HttpResponse::Unauthorized().body("Invalid token"))
}

pub(crate) fn authenticated_user_id(req: &HttpRequest) -> Result<i32, HttpResponse> {
    authenticated_user(req).map(|user| user.user_id)
}

#[cfg(test)]
mod tests {
    use super::{authenticated_user, authenticated_user_id};
    use crate::domain::identity::UserJWT;
    use actix_web::{body::to_bytes, http::StatusCode, test as actix_test, HttpMessage};

    #[test]
    fn returns_user_from_request_extensions() {
        let req = actix_test::TestRequest::default().to_http_request();
        req.extensions_mut().insert(UserJWT {
            user_id: 42,
            exp: 1000,
        });

        let user = match authenticated_user(&req) {
            Ok(user) => user,
            Err(response) => panic!("expected user from extensions, got {}", response.status()),
        };

        assert_eq!(user.user_id, 42);
        assert_eq!(user.exp, 1000);
    }

    #[test]
    fn returns_user_id_from_request_extensions() {
        let req = actix_test::TestRequest::default().to_http_request();
        req.extensions_mut().insert(UserJWT {
            user_id: 7,
            exp: 1000,
        });

        let user_id = match authenticated_user_id(&req) {
            Ok(user_id) => user_id,
            Err(response) => panic!(
                "expected user id from extensions, got {}",
                response.status()
            ),
        };

        assert_eq!(user_id, 7);
    }

    #[actix_web::test]
    async fn rejects_missing_authorization_header() {
        let req = actix_test::TestRequest::default().to_http_request();

        let response = match authenticated_user(&req) {
            Ok(user) => panic!("expected unauthorized response, got user {}", user.user_id),
            Err(response) => response,
        };

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let body = to_bytes(response.into_body()).await.unwrap();
        assert_eq!(body.as_ref(), b"Missing Authorization header");
    }

    #[actix_web::test]
    async fn rejects_authorization_header_without_bearer_prefix() {
        let req = actix_test::TestRequest::default()
            .insert_header(("Authorization", "Token abc123"))
            .to_http_request();

        let response = match authenticated_user(&req) {
            Ok(user) => panic!("expected unauthorized response, got user {}", user.user_id),
            Err(response) => response,
        };

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let body = to_bytes(response.into_body()).await.unwrap();
        assert_eq!(body.as_ref(), b"Invalid Authorization header format");
    }

    #[actix_web::test]
    async fn rejects_invalid_bearer_token() {
        let req = actix_test::TestRequest::default()
            .insert_header(("Authorization", "Bearer not-a-real-jwt"))
            .to_http_request();

        let response = match authenticated_user(&req) {
            Ok(user) => panic!("expected unauthorized response, got user {}", user.user_id),
            Err(response) => response,
        };

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let body = to_bytes(response.into_body()).await.unwrap();
        assert_eq!(body.as_ref(), b"Invalid token");
    }
}
