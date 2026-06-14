use std::cell::Cell;

use futures::future::{ready, BoxFuture, FutureExt};

use super::reset_password;
use crate::application::identity::reset_password::{
    ResetPasswordCommand, ResetPasswordError, ResetPasswordHasher, ResetPasswordOutcome,
    ResetPasswordStore,
};

#[derive(Default)]
struct FakeResetPasswordStore {
    outcome: ResetPasswordOutcome,
    calls: Vec<(String, String)>,
}

impl ResetPasswordStore for FakeResetPasswordStore {
    fn reset_password(
        &mut self,
        token: String,
        password_hash: String,
    ) -> BoxFuture<'_, Result<ResetPasswordOutcome, ResetPasswordError>> {
        self.calls.push((token, password_hash));
        ready(Ok(self.outcome)).boxed()
    }
}

struct FakePasswordHasher {
    result: Result<String, ResetPasswordError>,
    calls: Cell<u32>,
}

impl ResetPasswordHasher for FakePasswordHasher {
    fn hash_password(&self, _: &str) -> Result<String, ResetPasswordError> {
        self.calls.set(self.calls.get() + 1);
        self.result.clone()
    }
}

#[tokio::test]
async fn rejects_missing_token_before_side_effects() {
    let mut store = FakeResetPasswordStore::default();
    let hasher = hasher_ok();

    let error = reset_password(&mut store, &hasher, command("   ", "BetterPass123!"))
        .await
        .unwrap_err();

    assert_eq!(error, ResetPasswordError::MissingToken);
    assert_eq!(hasher.calls.get(), 0);
    assert!(store.calls.is_empty());
}

#[tokio::test]
async fn rejects_weak_password_before_side_effects() {
    let mut store = FakeResetPasswordStore::default();
    let hasher = hasher_ok();

    let error = reset_password(&mut store, &hasher, command("reset-token", "short"))
        .await
        .unwrap_err();

    assert_eq!(
        error,
        ResetPasswordError::InvalidPassword(
            "Password must be at least 12 characters long".to_string()
        )
    );
    assert_eq!(hasher.calls.get(), 0);
    assert!(store.calls.is_empty());
}

#[tokio::test]
async fn hashes_password_and_resets_token() {
    let mut store = FakeResetPasswordStore {
        outcome: ResetPasswordOutcome::Reset,
        calls: Vec::new(),
    };
    let hasher = hasher_ok();

    let outcome = reset_password(
        &mut store,
        &hasher,
        command(" reset-token ", "BetterPass123!"),
    )
    .await
    .unwrap();

    assert_eq!(outcome, ResetPasswordOutcome::Reset);
    assert_eq!(hasher.calls.get(), 1);
    assert_eq!(
        store.calls,
        vec![("reset-token".to_string(), "hashed-password".to_string())]
    );
}

fn command(token: &str, password: &str) -> ResetPasswordCommand {
    ResetPasswordCommand {
        token: token.to_string(),
        password: password.to_string(),
    }
}

fn hasher_ok() -> FakePasswordHasher {
    FakePasswordHasher {
        result: Ok("hashed-password".to_string()),
        calls: Cell::new(0),
    }
}
