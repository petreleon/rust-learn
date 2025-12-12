// Filename: organization_hierarchy_middleware.rs

use actix_service::{forward_ready, Service, Transform};
use actix_web::{dev::{ServiceRequest, ServiceResponse}, web, Error};
use futures::future::{self, Ready, LocalBoxFuture};
use futures::FutureExt;
use std::cmp::Ordering;
use std::marker::PhantomData;

// Assuming these modules are defined in your application
use crate::db::DbPool;
use crate::models::param_type::ParamType;
use crate::utils::{request_utils::extract_param, db_utils::organization::user_hierarchy_compare_organization};

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
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static + std::clone::Clone,
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

    // These clones ensure that we own the data fully and no references are held.
        let organization_id_str_opt = extract_param(&req, "organization_id", ParamType::Query).map(|s| s.to_owned());
        let second_user_id_str_opt = extract_param(&req, "user_id", ParamType::Query).map(|s| s.to_owned());

        // Clone `self` if needed or ensure `self.service` is moved or referenced correctly.
        let service = self.service.clone();
        let mut can_proceed = false;
        async move {
            // Obtain a DB connection directly from the pool (no request extensions)
            let pool = req.app_data::<web::Data<DbPool>>().ok_or_else(|| actix_web::error::ErrorInternalServerError("Failed to get database pool"))?;
            let mut conn = pool.get().await.map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get database connection"))?;

            let organization_id = organization_id_str_opt
                .and_then(|id_str| id_str.parse::<i32>().ok())
                .ok_or_else(|| actix_web::error::ErrorBadRequest("Invalid or missing organization parameter"))?;

            let second_user_id = second_user_id_str_opt
                .and_then(|id_str| id_str.parse::<i32>().ok())
                .ok_or_else(|| actix_web::error::ErrorBadRequest("Invalid or missing user parameter"))?;

            // Extract and decode JWT from Authorization header rather than from request extensions
            let user_jwt = if let Some(auth_header) = req.headers().get("Authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = &auth_str["Bearer ".len()..];
                        match crate::utils::jwt_utils::decode_jwt(token) {
                            Ok(token_data) => token_data.claims,
                            Err(_) => return Err(actix_web::error::ErrorUnauthorized("Unauthorized")),
                        }
                    } else {
                        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
                    }
                } else {
                    return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
                }
            } else {
                return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
            };

            // Now call the compare function with the borrowed connection
            can_proceed = match user_hierarchy_compare_organization(&mut conn, organization_id, user_jwt.user_id, second_user_id).await {
                Ok(ordering) => ordering != Ordering::Less,
                Err(_) => return Err(actix_web::error::ErrorInternalServerError("Failed to compare user hierarchy with organization")),
            };
            if !can_proceed {
                return Err(actix_web::error::ErrorForbidden("Forbidden"));
            }

            // Use the cloned service here.
            let fut = service.call(req);
            fut.await
        }
        .boxed_local()
    }
}
