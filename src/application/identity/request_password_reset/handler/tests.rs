use std::cell::{Cell, RefCell};

use futures::future::{ready, BoxFuture, FutureExt};

use super::request_password_reset;
use crate::application::identity::request_password_reset::{
    PasswordResetEmailSender, PasswordResetRecipient, PasswordResetTokenGenerator,
    RequestPasswordResetCommand, RequestPasswordResetError, RequestPasswordResetOutcome,
    RequestPasswordResetStore,
};

#[derive(Default)]
struct FakeRequestPasswordResetStore {
    recipient: Option<PasswordResetRecipient>,
    lookups: Vec<String>,
    stored_tokens: Vec<(i32, String)>,
}

impl RequestPasswordResetStore for FakeRequestPasswordResetStore {
    fn find_user_by_email(
        &mut self,
        email: String,
    ) -> BoxFuture<'_, Result<Option<PasswordResetRecipient>, RequestPasswordResetError>> {
        self.lookups.push(email);
        ready(Ok(self.recipient.clone())).boxed()
    }

    fn store_reset_token(
        &mut self,
        user_id: i32,
        token: String,
    ) -> BoxFuture<'_, Result<(), RequestPasswordResetError>> {
        self.stored_tokens.push((user_id, token));
        ready(Ok(())).boxed()
    }
}

struct FakeTokenGenerator {
    token: Result<String, RequestPasswordResetError>,
    calls: Cell<u32>,
}

impl PasswordResetTokenGenerator for FakeTokenGenerator {
    fn generate_token(&self) -> Result<String, RequestPasswordResetError> {
        self.calls.set(self.calls.get() + 1);
        self.token.clone()
    }
}

#[derive(Default)]
struct FakeEmailSender {
    sent: RefCell<Vec<(String, String, String)>>,
}

impl PasswordResetEmailSender for FakeEmailSender {
    fn send_password_reset_email(&self, email: &str, name: &str, token: &str) {
        self.sent
            .borrow_mut()
            .push((email.to_string(), name.to_string(), token.to_string()));
    }
}

#[tokio::test]
async fn unknown_email_has_no_side_effects() {
    let mut store = FakeRequestPasswordResetStore::default();
    let token_generator = token_generator("reset-token");
    let email_sender = FakeEmailSender::default();

    let outcome = request_password_reset(&mut store, &token_generator, &email_sender, command())
        .await
        .unwrap();

    assert_eq!(outcome, RequestPasswordResetOutcome::UnknownEmail);
    assert_eq!(store.lookups, vec!["learner@example.com"]);
    assert!(store.stored_tokens.is_empty());
    assert_eq!(token_generator.calls.get(), 0);
    assert!(email_sender.sent.borrow().is_empty());
}

#[tokio::test]
async fn known_email_stores_token_and_sends_email() {
    let mut store = FakeRequestPasswordResetStore {
        recipient: Some(recipient()),
        ..Default::default()
    };
    let token_generator = token_generator("reset-token");
    let email_sender = FakeEmailSender::default();

    let outcome = request_password_reset(&mut store, &token_generator, &email_sender, command())
        .await
        .unwrap();

    assert_eq!(outcome, RequestPasswordResetOutcome::Sent);
    assert_eq!(store.stored_tokens, vec![(7, "reset-token".to_string())]);
    assert_eq!(
        email_sender.sent.borrow().as_slice(),
        [(
            "learner@example.com".to_string(),
            "Rust Learner".to_string(),
            "reset-token".to_string()
        )]
    );
}

#[tokio::test]
async fn token_generation_error_stops_before_store_and_email() {
    let expected = RequestPasswordResetError::TokenGeneration("rng failed".to_string());
    let mut store = FakeRequestPasswordResetStore {
        recipient: Some(recipient()),
        ..Default::default()
    };
    let token_generator = FakeTokenGenerator {
        token: Err(expected.clone()),
        calls: Cell::new(0),
    };
    let email_sender = FakeEmailSender::default();

    let error = request_password_reset(&mut store, &token_generator, &email_sender, command())
        .await
        .unwrap_err();

    assert_eq!(error, expected);
    assert!(store.stored_tokens.is_empty());
    assert!(email_sender.sent.borrow().is_empty());
}

fn command() -> RequestPasswordResetCommand {
    RequestPasswordResetCommand {
        email: "learner@example.com".to_string(),
    }
}

fn recipient() -> PasswordResetRecipient {
    PasswordResetRecipient {
        user_id: 7,
        email: "learner@example.com".to_string(),
        name: "Rust Learner".to_string(),
    }
}

fn token_generator(token: &str) -> FakeTokenGenerator {
    FakeTokenGenerator {
        token: Ok(token.to_string()),
        calls: Cell::new(0),
    }
}
