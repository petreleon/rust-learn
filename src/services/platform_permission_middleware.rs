use actix_service::{forward_ready, Service, Transform};
use actix_web::{dev::{ServiceRequest, ServiceResponse}, Error, web};
use futures::future::{self, Ready, LocalBoxFuture};
use std::marker::PhantomData;
use futures::FutureExt;
use actix_web::HttpMessage;

use crate::{db::DbPool, models::user_jwt::UserJWT, utils::db_utils::user_permission_platform_request};

pub struct PlatformPermissionMiddleware<S> {
    _service: PhantomData<S>,
    permission_name: String,
}

impl<S> PlatformPermissionMiddleware<S> {
    pub fn new(permission_name: String) -> Self {
        PlatformPermissionMiddleware {
            _service: PhantomData,
            permission_name,
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for PlatformPermissionMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = PlatformPermissionMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ready(Ok(PlatformPermissionMiddlewareService {
            service,
            permission_name: self.permission_name.clone(),
        }))
    }
}

pub struct PlatformPermissionMiddlewareService<S> {
    service: S,
    permission_name: String,
}

impl<S, B> Service<ServiceRequest> for PlatformPermissionMiddlewareService<S>
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
        let db_pool = match req.app_data::<web::Data<DbPool>>() {
            Some(pool) => pool.clone(),
            None => {
                let error = actix_web::error::ErrorInternalServerError("Failed to access database pool");
                return future::ready(Err(error)).boxed_local();
            },
        };

        let user_jwt_opt = req.extensions().get::<UserJWT>().cloned();
        let permission_name = self.permission_name.clone();

        // Now `req` can be moved without issues
        let fut = self.service.call(req);

        async move {
            let user_jwt = match user_jwt_opt {
                Some(u) => u,
                None => return Err(actix_web::error::ErrorUnauthorized("Unauthorized access")),
            };

            let mut conn = match db_pool.get() {
                Ok(conn) => conn,
                Err(_) => return Err(actix_web::error::ErrorInternalServerError("Failed to get database connection")),
            };

            match user_permission_platform_request(&mut conn, user_jwt.user_id, &permission_name) {
                Ok(has_permission) => {
                    if !has_permission {
                        return Err(actix_web::error::ErrorForbidden("User does not have the required permission"));
                    }
                },
                Err(_) => return Err(actix_web::error::ErrorInternalServerError("Failed to check user permission")),
            }

            fut.await
        }.boxed_local()
    }
}
