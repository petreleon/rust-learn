// src/http/middlewares/jwt_middleware.rs
use actix_service::Service;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    web, Error, HttpMessage,
};
use futures::future::{ok, ready, Either, Ready};
use std::task::{Context, Poll};

use crate::application::identity::auth_token::{
    AuthTokenVerificationError, AuthTokenVerifierService,
};

pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddlewareService { service })
    }
}

pub struct JwtMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if let Some(auth_header) = req.headers().get("Authorization") {
            let auth_str = match auth_header.to_str() {
                Ok(auth_str) => auth_str,
                Err(_) => {
                    return Either::Right(ready(Err(ErrorUnauthorized(
                        "Invalid Authorization header format",
                    ))))
                }
            };

            let Some(token) = auth_str.strip_prefix("Bearer ") else {
                return Either::Right(ready(Err(ErrorUnauthorized(
                    "Invalid Authorization header format",
                ))));
            };
            let Some(verifier) = req
                .app_data::<web::Data<AuthTokenVerifierService>>()
                .map(|data| data.get_ref().clone())
            else {
                return Either::Right(ready(Err(ErrorUnauthorized("Invalid token"))));
            };

            let user_jwt = match verifier.verify_token(token) {
                Ok(claims) => claims,
                Err(AuthTokenVerificationError::Expired) => {
                    return Either::Right(ready(Err(ErrorUnauthorized("Token expired"))));
                }
                Err(AuthTokenVerificationError::Invalid) => {
                    return Either::Right(ready(Err(ErrorUnauthorized("Invalid token"))));
                }
            };

            req.extensions_mut().insert(user_jwt);
        }

        Either::Left(self.service.call(req))
    }
}

#[cfg(test)]
mod tests;
