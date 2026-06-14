use std::cell::{Cell, RefCell};

use futures::future::{ready, BoxFuture, FutureExt};

use super::resend_verification;
use crate::application::identity::resend_verification::{
    ResendVerificationCommand, ResendVerificationError, ResendVerificationOutcome,
    ResendVerificationStore, VerificationEmailSender, VerificationEmailTarget,
    VerificationTokenGenerator,
};

#[derive(Default)]
struct FakeResendVerificationStore {
    target: Option<VerificationEmailTarget>,
    lookups: Vec<String>,
    rotations: Vec<(i32, String)>,
}

impl ResendVerificationStore for FakeResendVerificationStore {
    fn find_user_by_email(
        &mut self,
        email: String,
    ) -> BoxFuture<'_, Result<Option<VerificationEmailTarget>, ResendVerificationError>> {
        self.lookups.push(email);
        ready(Ok(self.target.clone())).boxed()
    }

    fn rotate_verification_token(
        &mut self,
        user_id: i32,
        token: String,
    ) -> BoxFuture<'_, Result<(), ResendVerificationError>> {
        self.rotations.push((user_id, token));
        ready(Ok(())).boxed()
    }
}

struct FakeTokenGenerator {
    token: Result<String, ResendVerificationError>,
    calls: Cell<u32>,
}

impl VerificationTokenGenerator for FakeTokenGenerator {
    fn generate_token(&self) -> Result<String, ResendVerificationError> {
        self.calls.set(self.calls.get() + 1);
        self.token.clone()
    }
}

#[derive(Default)]
struct FakeEmailSender {
    sent: RefCell<Vec<(String, String, String)>>,
}

impl VerificationEmailSender for FakeEmailSender {
    fn send_verification_email(&self, email: &str, name: &str, token: &str) {
        self.sent
            .borrow_mut()
            .push((email.to_string(), name.to_string(), token.to_string()));
    }
}

#[tokio::test]
async fn unknown_email_has_no_side_effects() {
    let mut store = FakeResendVerificationStore::default();
    let token_generator = token_generator("new-token");
    let email_sender = FakeEmailSender::default();

    let outcome = resend_verification(&mut store, &token_generator, &email_sender, command())
        .await
        .unwrap();

    assert_eq!(outcome, ResendVerificationOutcome::UnknownEmail);
    assert_eq!(store.lookups, vec!["learner@example.com"]);
    assert!(store.rotations.is_empty());
    assert_eq!(token_generator.calls.get(), 0);
    assert!(email_sender.sent.borrow().is_empty());
}

#[tokio::test]
async fn verified_email_has_no_side_effects() {
    let mut store = FakeResendVerificationStore {
        target: Some(target(true)),
        ..Default::default()
    };
    let token_generator = token_generator("new-token");
    let email_sender = FakeEmailSender::default();

    let outcome = resend_verification(&mut store, &token_generator, &email_sender, command())
        .await
        .unwrap();

    assert_eq!(outcome, ResendVerificationOutcome::AlreadyVerified);
    assert!(store.rotations.is_empty());
    assert_eq!(token_generator.calls.get(), 0);
    assert!(email_sender.sent.borrow().is_empty());
}

#[tokio::test]
async fn unverified_email_rotates_token_and_sends_email() {
    let mut store = FakeResendVerificationStore {
        target: Some(target(false)),
        ..Default::default()
    };
    let token_generator = token_generator("new-token");
    let email_sender = FakeEmailSender::default();

    let outcome = resend_verification(&mut store, &token_generator, &email_sender, command())
        .await
        .unwrap();

    assert_eq!(outcome, ResendVerificationOutcome::Sent);
    assert_eq!(store.rotations, vec![(7, "new-token".to_string())]);
    assert_eq!(
        email_sender.sent.borrow().as_slice(),
        [(
            "learner@example.com".to_string(),
            "Rust Learner".to_string(),
            "new-token".to_string()
        )]
    );
}

#[tokio::test]
async fn token_generation_error_stops_before_store_and_email() {
    let expected = ResendVerificationError::TokenGeneration("rng failed".to_string());
    let mut store = FakeResendVerificationStore {
        target: Some(target(false)),
        ..Default::default()
    };
    let token_generator = FakeTokenGenerator {
        token: Err(expected.clone()),
        calls: Cell::new(0),
    };
    let email_sender = FakeEmailSender::default();

    let error = resend_verification(&mut store, &token_generator, &email_sender, command())
        .await
        .unwrap_err();

    assert_eq!(error, expected);
    assert!(store.rotations.is_empty());
    assert!(email_sender.sent.borrow().is_empty());
}

fn command() -> ResendVerificationCommand {
    ResendVerificationCommand {
        email: "learner@example.com".to_string(),
    }
}

fn target(email_verified: bool) -> VerificationEmailTarget {
    VerificationEmailTarget {
        user_id: 7,
        email: "learner@example.com".to_string(),
        name: "Rust Learner".to_string(),
        email_verified,
    }
}

fn token_generator(token: &str) -> FakeTokenGenerator {
    FakeTokenGenerator {
        token: Ok(token.to_string()),
        calls: Cell::new(0),
    }
}
