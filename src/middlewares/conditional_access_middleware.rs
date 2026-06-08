use actix_service::{forward_ready, Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error,
};
use futures::future::{self, LocalBoxFuture, Ready};
use futures::FutureExt;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;

type AccessCheck =
    dyn Fn(&ServiceRequest) -> LocalBoxFuture<'static, Result<bool, Error>> + Send + Sync;
type DenialError = dyn Fn() -> Error + Send + Sync;

// Middleware definition
pub struct ConditionalAccessMiddleware<S> {
    _service: PhantomData<S>,
    permitting_function: Arc<AccessCheck>,
    denial_error: Arc<DenialError>,
}

impl<S> ConditionalAccessMiddleware<S> {
    pub fn new<F, E>(permitting_function: F, denial_error: E) -> Self
    where
        F: Fn(&ServiceRequest) -> LocalBoxFuture<'static, Result<bool, Error>>
            + 'static
            + Send
            + Sync,
        E: Fn() -> Error + 'static + Send + Sync,
    {
        ConditionalAccessMiddleware {
            _service: PhantomData,
            permitting_function: Arc::new(permitting_function),
            denial_error: Arc::new(denial_error),
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
            service: Rc::new(service),
            permitting_function: self.permitting_function.clone(),
            denial_error: self.denial_error.clone(),
        }))
    }
}

pub struct ConditionalAccessMiddlewareService<S> {
    service: Rc<S>,
    permitting_function: Arc<AccessCheck>,
    denial_error: Arc<DenialError>,
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
        let check_fut = (self.permitting_function)(&req);
        let service = Rc::clone(&self.service);
        let denial_error = self.denial_error.clone();

        async move {
            match check_fut.await {
                Ok(true) => service.call(req).await,
                Ok(false) => Err((denial_error)()),
                Err(e) => Err(e),
            }
        }
        .boxed_local()
    }
}

#[cfg(test)]
mod tests {
    use super::ConditionalAccessMiddlewareService;
    use actix_service::Service;
    use actix_web::{
        body::BoxBody,
        dev::{ServiceRequest, ServiceResponse},
        http::StatusCode,
        test, Error, HttpResponse,
    };
    use futures::future;
    use std::cell::Cell;
    use std::rc::Rc;
    use std::sync::Arc;
    use std::task::{Context, Poll};

    struct CountingService {
        calls: Rc<Cell<usize>>,
    }

    impl Service<ServiceRequest> for CountingService {
        type Response = ServiceResponse<BoxBody>;
        type Error = Error;
        type Future = future::Ready<Result<Self::Response, Self::Error>>;

        fn poll_ready(&self, _ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&self, req: ServiceRequest) -> Self::Future {
            self.calls.set(self.calls.get() + 1);
            future::ready(Ok(req.into_response(HttpResponse::Ok().finish())))
        }
    }

    #[actix_web::test]
    async fn denied_requests_do_not_call_downstream_service() {
        let calls = Rc::new(Cell::new(0));
        let service = ConditionalAccessMiddlewareService {
            service: Rc::new(CountingService {
                calls: Rc::clone(&calls),
            }),
            permitting_function: Arc::new(|_req| Box::pin(future::ready(Ok(false)))),
            denial_error: Arc::new(|| actix_web::error::ErrorForbidden("denied")),
        };
        let req = test::TestRequest::default().to_srv_request();

        let result = service.call(req).await;

        assert_eq!(calls.get(), 0);
        match result {
            Ok(_) => panic!("denied request should not reach downstream service"),
            Err(error) => assert_eq!(error.error_response().status(), StatusCode::FORBIDDEN),
        }
    }

    #[actix_web::test]
    async fn permitted_requests_call_downstream_service_once() {
        let calls = Rc::new(Cell::new(0));
        let service = ConditionalAccessMiddlewareService {
            service: Rc::new(CountingService {
                calls: Rc::clone(&calls),
            }),
            permitting_function: Arc::new(|_req| Box::pin(future::ready(Ok(true)))),
            denial_error: Arc::new(|| actix_web::error::ErrorForbidden("denied")),
        };
        let req = test::TestRequest::default().to_srv_request();

        let response = match service.call(req).await {
            Ok(response) => response,
            Err(error) => panic!("permitted request failed: {error}"),
        };

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(calls.get(), 1);
    }
}
