// src/services/jwt_middleware.rs
use actix_service::Service;
use actix_web::{dev::{ServiceRequest, ServiceResponse, Transform}, error::ErrorBadRequest, web::Data, Error, HttpMessage};
use futures::future::{ok, ready, Either, Ready};
use std::task::{Context, Poll};

use crate::utils::jwt_utils::decode_jwt;
use crate::models::user_jwt::UserJWT;

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
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str["Bearer ".len()..];
                    if let Ok(token_data) = decode_jwt(token) {
                        let user_jwt: UserJWT = token_data.claims;
                        // Add user_jwt to request extensions
                        let exp = user_jwt.exp;
                        let now = chrono::Utc::now().timestamp() as usize; // Convert now to usize
                        if exp < now {
                            return Either::Right(ready(Err(ErrorBadRequest("Token expired"))));
                        }
                        req.extensions_mut().insert(user_jwt);
                    }
                }
            }
        }

        Either::Left(self.service.call(req))
    }
}
