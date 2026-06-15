use futures::future::BoxFuture;

pub const READINESS_DEPENDENCY_BLOCKCHAIN: &str = "blockchain";
pub const READINESS_DEPENDENCY_DATABASE: &str = "database";
pub const READINESS_DEPENDENCY_OBJECT_STORAGE: &str = "object_storage";

pub trait ReadinessDependency: Send {
    fn name(&self) -> &'static str;
    fn check(&mut self) -> BoxFuture<'_, Result<(), String>>;
}
