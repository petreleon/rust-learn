use futures::future::BoxFuture;

pub trait ReadinessDependency: Send {
    fn name(&self) -> &'static str;
    fn check(&mut self) -> BoxFuture<'_, Result<(), String>>;
}
