use super::auth_scope;
use crate::application::identity::password_policy::{
    validate_password_strength, PASSWORD_TOO_LONG_MESSAGE,
};
use crate::domain::identity::UserJWT;
use crate::infra::tokens::jwt::create_jwt;
use actix_web::{http::StatusCode, test as actix_test, App, HttpMessage};

#[test]
fn accepts_strong_password() {
    assert!(validate_password_strength("CorrectHorse1!").is_ok());
}

#[test]
fn accepts_minimum_length_password_with_required_character_classes() {
    assert!(validate_password_strength("Aa1!aaaaaaaa").is_ok());
}

#[test]
fn rejects_short_password() {
    assert_eq!(
        validate_password_strength("Aa1!").unwrap_err(),
        "Password must be at least 12 characters long"
    );
}

#[test]
fn rejects_passwords_missing_required_character_classes() {
    let cases = [
        "CORRECTHORSE1!",
        "correcthorse1!",
        "CorrectHorse!!",
        "CorrectHorse12",
    ];

    for password in cases {
        assert_eq!(
            validate_password_strength(password).unwrap_err(),
            "Password must include lowercase, uppercase, numeric, and symbol characters"
        );
    }
}

#[test]
fn rejects_passwords_over_bcrypt_byte_limit() {
    let long_password = format!("Aa1!{}", "a".repeat(68));

    assert_eq!(
        validate_password_strength(&long_password).unwrap_err(),
        PASSWORD_TOO_LONG_MESSAGE
    );
}

#[test]
fn accepts_password_at_bcrypt_byte_limit() {
    let password = format!("Aa1!{}", "a".repeat(67));

    assert_eq!(password.len(), 71);
    assert!(validate_password_strength(&password).is_ok());
}

#[actix_web::test]
async fn user_id_requires_authorization_header() {
    let app = actix_test::init_service(App::new().service(auth_scope())).await;

    let response = actix_test::call_service(
        &app,
        actix_test::TestRequest::get()
            .uri("/auth/user_id")
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let body = actix_test::read_body(response).await;
    assert_eq!(body.as_ref(), b"Missing Authorization header");
}

#[actix_web::test]
async fn user_id_rejects_malformed_authorization_header() {
    let app = actix_test::init_service(App::new().service(auth_scope())).await;

    let response = actix_test::call_service(
        &app,
        actix_test::TestRequest::get()
            .uri("/auth/user_id")
            .insert_header(("Authorization", "Basic not-a-bearer-token"))
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let body = actix_test::read_body(response).await;
    assert_eq!(body.as_ref(), b"Invalid Authorization header format");
}

#[actix_web::test]
async fn user_id_rejects_invalid_bearer_token() {
    let app = actix_test::init_service(App::new().service(auth_scope())).await;

    let response = actix_test::call_service(
        &app,
        actix_test::TestRequest::get()
            .uri("/auth/user_id")
            .insert_header(("Authorization", "Bearer not-a-valid-token"))
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let body = actix_test::read_body(response).await;
    assert_eq!(body.as_ref(), b"Invalid token");
}

#[actix_web::test]
async fn user_id_returns_id_for_valid_bearer_token() {
    let _ = dotenvy::dotenv();
    let token = create_jwt(42).expect("test JWT should be created");
    let app = actix_test::init_service(App::new().service(auth_scope())).await;

    let response = actix_test::call_service(
        &app,
        actix_test::TestRequest::get()
            .uri("/auth/user_id")
            .insert_header(("Authorization", format!("Bearer {token}")))
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);
    let body = actix_test::read_body(response).await;
    assert_eq!(body.as_ref(), b"Hello! Your ID is 42");
}

#[actix_web::test]
async fn user_id_uses_decoded_request_extension() {
    let app = actix_test::init_service(App::new().service(auth_scope())).await;
    let request = actix_test::TestRequest::get()
        .uri("/auth/user_id")
        .to_request();
    request.extensions_mut().insert(UserJWT::new(
        77,
        chrono::Utc::now() + chrono::Duration::hours(1),
    ));

    let response = actix_test::call_service(&app, request).await;

    assert_eq!(response.status(), StatusCode::OK);
    let body = actix_test::read_body(response).await;
    assert_eq!(body.as_ref(), b"Hello! Your ID is 77");
}
