use futures::future::{ready, BoxFuture, FutureExt};

use super::*;

struct FakeVerifyEmailStore {
    tokens: Vec<String>,
    result: Result<VerifyEmailOutcome, VerifyEmailError>,
}

impl VerifyEmailStore for FakeVerifyEmailStore {
    fn verify_email_token(
        &mut self,
        token: String,
    ) -> BoxFuture<'_, Result<VerifyEmailOutcome, VerifyEmailError>> {
        self.tokens.push(token);
        ready(self.result.clone()).boxed()
    }
}

#[tokio::test]
async fn delegates_token_verification_to_store() {
    let mut store = FakeVerifyEmailStore {
        tokens: Vec::new(),
        result: Ok(VerifyEmailOutcome::Verified),
    };

    let outcome = verify_email(&mut store, command("token-1")).await.unwrap();

    assert_eq!(outcome, VerifyEmailOutcome::Verified);
    assert_eq!(store.tokens, vec!["token-1"]);
}

#[tokio::test]
async fn returns_store_error() {
    let mut store = FakeVerifyEmailStore {
        tokens: Vec::new(),
        result: Err(VerifyEmailError::Database("failed".to_string())),
    };

    let error = verify_email(&mut store, command("token-1"))
        .await
        .unwrap_err();

    assert_eq!(error, VerifyEmailError::Database("failed".to_string()));
}

fn command(token: &str) -> VerifyEmailCommand {
    VerifyEmailCommand {
        token: token.to_string(),
    }
}
