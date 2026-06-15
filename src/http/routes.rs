use actix_service::ServiceFactory;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, Scope};

use crate::http::middlewares::{
    conditional_access_middleware::ConditionalAccessMiddleware, jwt_middleware::JwtMiddleware,
};
use actix_web::web;

pub fn api_scope() -> Scope<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse,
        Error = Error,
        InitError = (),
    >,
> {
    web::scope("/api")
        .wrap(JwtMiddleware)
        .wrap(ConditionalAccessMiddleware::new(
            |_req: &ServiceRequest| Box::pin(futures::future::ready(Ok(true))),
            || actix_web::error::ErrorUnauthorized("Denied by conditional middleware"),
        ))
        .configure(crate::http::identity::configure_routes)
        .configure(crate::http::notifications::configure_routes)
        .configure(crate::http::learning::configure_routes)
        .configure(crate::http::organizations::configure_routes)
        .configure(crate::http::reporting::configure_routes)
        .configure(crate::http::kyc::configure_routes)
        .configure(crate::http::rewards::configure_routes)
        .configure(crate::http::access_control::configure_routes)
        .configure(crate::http::teacher_applications::configure_routes)
        .configure(crate::http::wallet::configure_routes)
}
