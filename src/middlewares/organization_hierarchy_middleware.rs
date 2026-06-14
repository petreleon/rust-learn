// Filename: organization_hierarchy_middleware.rs

use actix_service::{forward_ready, Service, Transform};
use actix_web::HttpMessage;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    web, Error,
};
use futures::future::{self, LocalBoxFuture, Ready};
use futures::FutureExt;
use std::cmp::Ordering;
use std::marker::PhantomData;

use crate::db::DbPool;
use crate::http::request_params::extract_param;
use crate::http::request_params::ParamType;
use crate::infra::postgres::access_control::authorization_checks::user_hierarchy_compare_organization;
use crate::models::user_jwt::UserJWT;

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
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>
        + 'static
        + std::clone::Clone,
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

pub struct OrganizationHierarchyMiddlewareService<S: Clone> {
    service: S,
    type_param_of_id_user: ParamType,
    name_param_of_id_user: String,
    type_param_of_organization: ParamType,
    name_param_of_organization: String,
}

impl<S: Clone, B> Service<ServiceRequest> for OrganizationHierarchyMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + Clone + 'static,
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
                return future::ready(Err(actix_web::error::ErrorInternalServerError(
                    "Failed to access database pool",
                )))
                .boxed_local();
            }
        };

        let type_param_of_organization = self.type_param_of_organization;
        let name_param_of_organization = self.name_param_of_organization.clone();
        let type_param_of_id_user = self.type_param_of_id_user;
        let name_param_of_id_user = self.name_param_of_id_user.clone();
        let organization_id_str_opt = extract_param(
            &req,
            &name_param_of_organization,
            type_param_of_organization,
        )
        .map(|s| s.to_owned());
        let second_user_id_str_opt =
            extract_param(&req, &name_param_of_id_user, type_param_of_id_user)
                .map(|s| s.to_owned());
        let user_jwt_opt = req.extensions().get::<UserJWT>().cloned();

        let service = self.service.clone();
        async move {
            let organization_id = organization_id_str_opt
                .and_then(|id_str| id_str.parse::<i32>().ok())
                .ok_or_else(|| {
                    actix_web::error::ErrorBadRequest("Invalid or missing organization parameter")
                })?;

            let second_user_id = second_user_id_str_opt
                .and_then(|id_str| id_str.parse::<i32>().ok())
                .ok_or_else(|| {
                    actix_web::error::ErrorBadRequest("Invalid or missing user parameter")
                })?;

            let user_jwt = user_jwt_opt
                .ok_or_else(|| actix_web::error::ErrorUnauthorized("Unauthorized access"))?;

            let mut conn = db_pool.get().await.map_err(|_| {
                actix_web::error::ErrorInternalServerError("Failed to get database connection")
            })?;

            let can_proceed = match user_hierarchy_compare_organization(
                &mut conn,
                organization_id,
                user_jwt.user_id,
                second_user_id,
            )
            .await
            {
                Ok(ordering) => ordering != Ordering::Less,
                Err(_) => {
                    return Err(actix_web::error::ErrorInternalServerError(
                        "Failed to compare user hierarchy with organization",
                    ))
                }
            };
            if !can_proceed {
                return Err(actix_web::error::ErrorForbidden("Forbidden"));
            }

            let fut = service.call(req);
            fut.await
        }
        .boxed_local()
    }
}
