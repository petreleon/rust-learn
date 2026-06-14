use std::cell::{Cell, RefCell};

use chrono::NaiveDate;
use futures::future::{ready, BoxFuture, FutureExt};

use super::register;
use crate::application::identity::register::{
    RegisterCommand, RegisterError, RegisterOutcome, RegisterStore, RegisteredUser,
    RegistrationAccount, RegistrationEmailSender, RegistrationPasswordHasher,
    RegistrationTokenGenerator,
};

struct FakeRegisterStore {
    result: Result<RegisteredUser, RegisterError>,
    calls: Vec<(RegistrationAccount, String, String)>,
}

impl RegisterStore for FakeRegisterStore {
    fn register_account(
        &mut self,
        account: RegistrationAccount,
        password_hash: String,
        verification_token: String,
    ) -> BoxFuture<'_, Result<RegisteredUser, RegisterError>> {
        self.calls
            .push((account, password_hash, verification_token));
        ready(self.result.clone()).boxed()
    }
}

struct FakePasswordHasher {
    result: Result<String, RegisterError>,
    calls: Cell<u32>,
}

impl RegistrationPasswordHasher for FakePasswordHasher {
    fn hash_password(&self, _: &str) -> Result<String, RegisterError> {
        self.calls.set(self.calls.get() + 1);
        self.result.clone()
    }
}

struct FakeTokenGenerator {
    result: Result<String, RegisterError>,
    calls: Cell<u32>,
}

impl RegistrationTokenGenerator for FakeTokenGenerator {
    fn generate_token(&self) -> Result<String, RegisterError> {
        self.calls.set(self.calls.get() + 1);
        self.result.clone()
    }
}

#[derive(Default)]
struct FakeEmailSender {
    sent: RefCell<Vec<(String, String, String)>>,
}

impl RegistrationEmailSender for FakeEmailSender {
    fn send_verification_email(&self, email: &str, name: &str, token: &str) {
        self.sent
            .borrow_mut()
            .push((email.to_string(), name.to_string(), token.to_string()));
    }
}

#[tokio::test]
async fn rejects_weak_password_before_side_effects() {
    let mut store = store_ok();
    let hasher = hasher_ok();
    let token_generator = token_generator_ok();
    let email_sender = FakeEmailSender::default();

    let error = register(
        &mut store,
        &hasher,
        &token_generator,
        &email_sender,
        command_with_password("short"),
    )
    .await
    .unwrap_err();

    assert_eq!(
        error,
        RegisterError::InvalidPassword("Password must be at least 12 characters long".to_string())
    );
    assert_eq!(hasher.calls.get(), 0);
    assert_eq!(token_generator.calls.get(), 0);
    assert!(store.calls.is_empty());
    assert!(email_sender.sent.borrow().is_empty());
}

#[tokio::test]
async fn creates_account_and_sends_verification_email() {
    let mut store = store_ok();
    let hasher = hasher_ok();
    let token_generator = token_generator_ok();
    let email_sender = FakeEmailSender::default();

    let outcome = register(
        &mut store,
        &hasher,
        &token_generator,
        &email_sender,
        command(),
    )
    .await
    .unwrap();

    assert_eq!(outcome, RegisterOutcome::Registered);
    assert_eq!(hasher.calls.get(), 1);
    assert_eq!(token_generator.calls.get(), 1);
    assert_eq!(
        store.calls,
        vec![(
            RegistrationAccount {
                email: "learner@example.com".to_string(),
                name: "Rust Learner".to_string(),
                date_of_birth: NaiveDate::from_ymd_opt(2001, 2, 3)
            },
            "hashed-password".to_string(),
            "verification-token".to_string()
        )]
    );
    assert_eq!(
        email_sender.sent.borrow().as_slice(),
        [(
            "learner@example.com".to_string(),
            "Rust Learner".to_string(),
            "verification-token".to_string()
        )]
    );
}

fn command() -> RegisterCommand {
    command_with_password("ValidPass123!")
}

fn command_with_password(password: &str) -> RegisterCommand {
    RegisterCommand {
        email: "learner@example.com".to_string(),
        password: password.to_string(),
        name: "Rust Learner".to_string(),
        date_of_birth: NaiveDate::from_ymd_opt(2001, 2, 3),
    }
}

fn store_ok() -> FakeRegisterStore {
    FakeRegisterStore {
        result: Ok(RegisteredUser {
            email: "learner@example.com".to_string(),
            name: "Rust Learner".to_string(),
        }),
        calls: Vec::new(),
    }
}

fn hasher_ok() -> FakePasswordHasher {
    FakePasswordHasher {
        result: Ok("hashed-password".to_string()),
        calls: Cell::new(0),
    }
}

fn token_generator_ok() -> FakeTokenGenerator {
    FakeTokenGenerator {
        result: Ok("verification-token".to_string()),
        calls: Cell::new(0),
    }
}
