use actix_service::{forward_ready, Service, Transform};
use actix_web::{dev::{ServiceRequest, ServiceResponse}, Error, web};
use futures::future::{self, Ready, LocalBoxFuture};
use std::marker::PhantomData;
use std::cmp::Ordering;
use futures::FutureExt;
use actix_web::HttpMessage;

use crate::{db::DbPool, utils::request_utils::extract_param};
use crate::models::{user_jwt::UserJWT, param_type::ParamType};
use crate::utils::db_utils::user_hierarchy_compare_platform;

pub struct PlatformHierarchyMiddleware<S> {
    _service: PhantomData<S>,
    type_param_of_id_user: ParamType,
    name_param_of_id_user: String,
}

impl<S> PlatformHierarchyMiddleware<S> {
    pub fn new(type_param_of_id_user: ParamType, name_param_of_id_user: String) -> Self {
        PlatformHierarchyMiddleware {
            _service: PhantomData,
            type_param_of_id_user,
            name_param_of_id_user,
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for PlatformHierarchyMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = PlatformHierarchyMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ready(Ok(PlatformHierarchyMiddlewareService {
            service,
            type_param_of_id_user: self.type_param_of_id_user,
            name_param_of_id_user: self.name_param_of_id_user.clone(),
        }))
    }
}

pub struct PlatformHierarchyMiddlewareService<S> {
    service: S,
    type_param_of_id_user: ParamType,
    name_param_of_id_user: String,
}

impl<S, B> Service<ServiceRequest> for PlatformHierarchyMiddlewareService<S>
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
        let type_param_of_id_user = self.type_param_of_id_user;
        let name_param_of_id_user = self.name_param_of_id_user.clone();
    
        // Extract necessary data from `req` before it's moved
        let second_user_id_str_opt = extract_param(&req, &name_param_of_id_user, type_param_of_id_user);
    
        // Now `req` can be moved without issues
        let fut = self.service.call(req);
    
        async move {
            // Use the extracted data instead of accessing `req` directly
            let second_user_id_str = match second_user_id_str_opt {
                Some(id_str) => id_str,
                None => return Err(actix_web::error::ErrorBadRequest("Invalid or missing parameter")),
            };
    
            let second_user_id = match second_user_id_str.parse::<i32>() {
                Ok(id) => id,
                Err(_) => return Err(actix_web::error::ErrorBadRequest("Invalid or missing parameter")),
            };
    
            let mut conn = match db_pool.get() {
                Ok(conn) => conn,
                Err(_) => return Err(actix_web::error::ErrorInternalServerError("Failed to get database connection")),
            };
    
            let user_jwt = match user_jwt_opt {
                Some(u) => u,
                None => return Err(actix_web::error::ErrorUnauthorized("Unauthorized access")),
            };
    
            match user_hierarchy_compare_platform(&mut conn, user_jwt.user_id, second_user_id) {
                Ok(ordering) => {
                    if ordering == Ordering::Less {
                        return Err(actix_web::error::ErrorForbidden("Modification not permitted: second user has a greater hierarchy level"));
                    }
                },
                Err(_) => return Err(actix_web::error::ErrorInternalServerError("Failed to compare user hierarchy")),
            }
    
            fut.await
        }.boxed_local()
    }
    
}
