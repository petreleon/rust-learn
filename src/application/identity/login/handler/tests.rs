use futures::future::{ready, BoxFuture, FutureExt};

use super::*;
use crate::application::identity::login::LoginAuthentication;

#[derive(Default)]
struct FakeLoginStore {
    authentication: Option<LoginAuthentication>,
}

impl LoginStore for FakeLoginStore {
    fn find_password_auth(
        &mut self,
        _: String,
    ) -> BoxFuture<'_, Result<Option<LoginAuthentication>, LoginError>> {
        ready(Ok(self.authentication.clone())).boxed()
    }
}

struct FakePasswordVerifier {
    matches: bool,
}

impl PasswordVerifier for FakePasswordVerifier {
    fn verify_password(&self, _: &str, _: &str) -> bool {
        self.matches
    }
}

struct FakeTokenIssuer;

impl LoginTokenIssuer for FakeTokenIssuer {
    fn issue_token(&self, user_id: i32) -> Result<String, LoginError> {
        Ok(format!("jwt-{user_id}"))
    }
}

#[tokio::test]
async fn returns_invalid_credentials_when_user_is_missing() {
    let mut store = FakeLoginStore::default();

    let error = login(&mut store, &verifier(true), &FakeTokenIssuer, command())
        .await
        .unwrap_err();

    assert_eq!(error, LoginError::InvalidCredentials);
}

#[tokio::test]
async fn returns_invalid_credentials_when_password_does_not_match() {
    let mut store = FakeLoginStore {
        authentication: Some(authentication(true, Some("hash"))),
    };

    let error = login(&mut store, &verifier(false), &FakeTokenIssuer, command())
        .await
        .unwrap_err();

    assert_eq!(error, LoginError::InvalidCredentials);
}

#[tokio::test]
async fn rejects_missing_password_authentication() {
    let mut store = FakeLoginStore {
        authentication: Some(authentication(true, None)),
    };

    let error = login(&mut store, &verifier(true), &FakeTokenIssuer, command())
        .await
        .unwrap_err();

    assert_eq!(error, LoginError::MissingPasswordAuthentication);
}

#[tokio::test]
async fn rejects_unverified_email_after_password_match() {
    let mut store = FakeLoginStore {
        authentication: Some(authentication(false, Some("hash"))),
    };

    let error = login(&mut store, &verifier(true), &FakeTokenIssuer, command())
        .await
        .unwrap_err();

    assert_eq!(error, LoginError::EmailUnverified { user_id: 7 });
}

#[tokio::test]
async fn returns_jwt_for_verified_matching_credentials() {
    let mut store = FakeLoginStore {
        authentication: Some(authentication(true, Some("hash"))),
    };

    let output = login(&mut store, &verifier(true), &FakeTokenIssuer, command())
        .await
        .unwrap();

    assert_eq!(output.jwt, "jwt-7");
}

fn verifier(matches: bool) -> FakePasswordVerifier {
    FakePasswordVerifier { matches }
}

fn command() -> LoginCommand {
    LoginCommand {
        email: "learner@example.com".to_string(),
        password: "CorrectHorse1!".to_string(),
    }
}

fn authentication(email_verified: bool, password_hash: Option<&str>) -> LoginAuthentication {
    LoginAuthentication {
        user_id: 7,
        email_verified,
        password_hash: password_hash.map(ToOwned::to_owned),
    }
}
