use actix_service::{forward_ready, Service, Transform};
use actix_web::{dev::{ServiceRequest, ServiceResponse}, Error, web};
use futures::future::{self, Ready, LocalBoxFuture, BoxFuture};
use std::marker::PhantomData;
use std::sync::Arc;
use futures::FutureExt;

// Middleware definition
pub struct ConditionalAccessMiddleware<S> {
    _service: PhantomData<S>,
    permitting_function: Arc<dyn Fn(&ServiceRequest) -> bool + Send + Sync>,
    denial_error: Arc<dyn Fn() -> Error + Send + Sync>, // Change here
}

impl<S> ConditionalAccessMiddleware<S> {
    pub fn new<F, E>(permitting_function: F, denial_error: E) -> Self
    where
        F: Fn(&ServiceRequest) -> bool + 'static + Send + Sync,
        E: Fn() -> Error + 'static + Send + Sync, // Change here
    {
        ConditionalAccessMiddleware {
            _service: PhantomData,
            permitting_function: Arc::new(permitting_function),
            denial_error: Arc::new(denial_error), // Change here
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for ConditionalAccessMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ConditionalAccessMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ready(Ok(ConditionalAccessMiddlewareService {
            service,
            permitting_function: self.permitting_function.clone(),
            denial_error: self.denial_error.clone(), // Change here
        }))
    }
}

pub struct ConditionalAccessMiddlewareService<S> {
    service: S,
    permitting_function: Arc<dyn Fn(&ServiceRequest) -> bool + Send + Sync>,
    denial_error: Arc<dyn Fn() -> Error + Send + Sync>, // Change here
}

impl<S, B> Service<ServiceRequest> for ConditionalAccessMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if (self.permitting_function)(&req) {
            self.service.call(req).boxed_local()
        } else {
            future::ready(Err((self.denial_error)())).boxed_local() // Change here
        }
    }
}
