use futures::future::BoxFuture;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JwksOutput {
    pub keys: Vec<JsonWebKeyOutput>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonWebKeyOutput {
    pub kty: String,
    pub public_key_use: String,
    pub kid: String,
    pub alg: String,
    pub n: String,
    pub e: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JwksError {
    message: String,
}

impl JwksError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

pub trait JwksUseCase: Send + Sync {
    fn jwks(&self) -> BoxFuture<'_, Result<JwksOutput, JwksError>>;
}
