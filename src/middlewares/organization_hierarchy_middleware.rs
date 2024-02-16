// Filename: organization_hierarchy_middleware.rs

use actix_service::{forward_ready, Service, Transform};
use actix_web::{dev::{ServiceRequest, ServiceResponse}, Error, web, HttpMessage};
use futures::future::{self, Ready, LocalBoxFuture};
use futures::FutureExt;
use std::cmp::Ordering;
use std::marker::PhantomData;

// Assuming these modules are defined in your application
use crate::db::DbPool;
use crate::models::{user_jwt::UserJWT, param_type::ParamType};
use crate::utils::{request_utils::extract_param, db_utils::user_hierarchy_compare_organization};

pub struct OrganizationHierarchyMiddleware<S> {
    _service: PhantomData<S>,
    type_param_of_id_user: ParamType,
    name_param_of_id_user: String,
    type_param_of_organization: ParamType,
    name_param_of_organization: String,
}

impl<S> OrganizationHierarchyMiddleware<S> {
    pub fn new(
        type_param_of_id_user: ParamType, 
        name_param_of_id_user: String,
        type_param_of_organization: ParamType,
        name_param_of_organization: String,
    ) -> Self {
        OrganizationHierarchyMiddleware {
            _service: PhantomData,
            type_param_of_id_user,
            name_param_of_id_user,
            type_param_of_organization,
            name_param_of_organization,
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for OrganizationHierarchyMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = OrganizationHierarchyMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ready(Ok(OrganizationHierarchyMiddlewareService {
            service,
            type_param_of_id_user: self.type_param_of_id_user,
            name_param_of_id_user: self.name_param_of_id_user.clone(),
            type_param_of_organization: self.type_param_of_organization,
            name_param_of_organization: self.name_param_of_organization.clone(),
        }))
    }
}

pub struct OrganizationHierarchyMiddlewareService<S> {
    service: S,
    type_param_of_id_user: ParamType,
    name_param_of_id_user: String,
    type_param_of_organization: ParamType,
    name_param_of_organization: String,
}

impl<S, B> Service<ServiceRequest> for OrganizationHierarchyMiddlewareService<S>
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
        let organization_id_str_opt = extract_param(&req, &self.name_param_of_organization, self.type_param_of_organization);
        let second_user_id_str_opt = extract_param(&req, &self.name_param_of_id_user, self.type_param_of_id_user);

        let fut = self.service.call(req);

        async move {
            let organization_id = match organization_id_str_opt {
                Some(id_str) => id_str.parse::<i32>().ok(),
                None => None,
            }.ok_or(actix_web::error::ErrorBadRequest("Invalid or missing organization parameter"))?;

            let second_user_id = match second_user_id_str_opt {
                Some(id_str) => id_str.parse::<i32>().ok(),
                None => None,
            }.ok_or(actix_web::error::ErrorBadRequest("Invalid or missing user parameter"))?;

            let mut conn = match db_pool.get() {
                Ok(conn) => conn,
                Err(_) => return Err(actix_web::error::ErrorInternalServerError("Failed to get database connection")),
            };

            let user_jwt = match user_jwt_opt {
                Some(u) => u,
                None => return Err(actix_web::error::ErrorUnauthorized("Unauthorized access")),
            };

            match user_hierarchy_compare_organization(&mut conn, organization_id, user_jwt.user_id, second_user_id) {
                Ok(ordering) => {
                    if ordering == Ordering::Less {
                        return Err(actix_web::error::ErrorForbidden("Modification not permitted: second user has a greater hierarchy level within the organization"));
                    }
                },
                Err(_) => return Err(actix_web::error::ErrorInternalServerError("Failed to compare user hierarchy with organization")),
            }

            fut.await
        }.boxed_local()
    }
}
