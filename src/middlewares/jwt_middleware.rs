// src/middlewares/jwt_middleware.rs
use actix_service::Service;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    Error, HttpMessage,
};
use futures::future::{ok, ready, Either, Ready};
use jsonwebtoken::errors::ErrorKind;
use std::task::{Context, Poll};

use crate::infra::tokens::jwt::decode_jwt;
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

            let token_data = match decode_jwt(token) {
                Ok(token_data) => token_data,
                Err(error) => {
                    let message = match error.kind() {
                        ErrorKind::ExpiredSignature => "Token expired",
                        _ => "Invalid token",
                    };
                    return Either::Right(ready(Err(ErrorUnauthorized(message))));
                }
            };

            let user_jwt: UserJWT = token_data.claims;
            let exp = user_jwt.exp;
            let now = chrono::Utc::now().timestamp() as usize;
            if exp < now {
                return Either::Right(ready(Err(ErrorUnauthorized("Token expired"))));
            }
            req.extensions_mut().insert(user_jwt);
        }

        Either::Left(self.service.call(req))
    }
}

#[cfg(test)]
mod tests;
