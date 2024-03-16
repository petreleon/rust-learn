// Filename: organization_hierarchy_middleware.rs

use actix_service::{forward_ready, Service, Transform};
use actix_web::{dev::{ServiceRequest, ServiceResponse}, error::{ErrorBadRequest, ErrorForbidden, ErrorInternalServerError, ErrorUnauthorized}, web, Error, HttpMessage};
use diesel::{connection, r2d2::{ConnectionManager, PooledConnection}, PgConnection};
use futures::future::{self, Ready, LocalBoxFuture};
use futures::FutureExt;
use std::{cell::Ref, cmp::Ordering};
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
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Directly extract the database connection from the request's extensions

        // Extract other necessary data from the request
        let user_jwt_opt = req.extensions().get::<UserJWT>().cloned();
        let organization_id_str_opt = extract_param(&req, "organization_id", ParamType::Query);
        let second_user_id_str_opt = extract_param(&req, "user_id", ParamType::Query);
        let db_pool = match req.app_data::<web::Data<DbPool>>() {
            Some(pool) => pool.clone(),
            None => {
                let error = actix_web::error::ErrorInternalServerError("Failed to access database pool");
                return future::ready(Err(error)).boxed_local();
            },
        };
        let fut = self.service.call(req);

        async move {
            
            let mut connection = db_pool.get().or_else(|_| Err(Error::from(ErrorInternalServerError("Failed to get database connection"))))?;
            let organization_id = organization_id_str_opt
                .and_then(|id_str| id_str.parse::<i32>().ok())
                .ok_or_else(|| ErrorBadRequest("Invalid or missing organization parameter"))?;

            let second_user_id = second_user_id_str_opt
                .and_then(|id_str| id_str.parse::<i32>().ok())
                .ok_or_else(|| ErrorBadRequest("Invalid or missing user parameter"))?;

            let user_jwt = user_jwt_opt.ok_or_else(|| ErrorUnauthorized("Unauthorized"))?;

            // Perform your logic with the database connection
            match user_hierarchy_compare_organization(&mut connection, organization_id, user_jwt.user_id, second_user_id) {
                Ok(ordering) => {
                    if ordering == Ordering::Less {
                        return Err(ErrorForbidden("Forbidden"));
                    }
                },
                Err(_) => return Err(ErrorInternalServerError("Failed to compare user hierarchy with organization")),
            }

            fut.await
        }.boxed_local()
    }
}
