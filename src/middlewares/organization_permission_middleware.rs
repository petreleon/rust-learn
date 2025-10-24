use actix_service::{forward_ready, Service, Transform};
use actix_web::{dev::{ServiceRequest, ServiceResponse}, Error, web, HttpMessage};
use futures::future::{self, Ready, LocalBoxFuture};
use futures::FutureExt;
use std::marker::PhantomData;

// Assuming these modules are defined in your application
use crate::db::DbPool;
use crate::models::{user_jwt::UserJWT, param_type::ParamType};
use crate::utils::{request_utils::extract_param, db_utils::organization::user_permission_organization_request};

pub struct OrganizationPermissionMiddleware<S> {
    _service: PhantomData<S>,
    permission_name: String,
    type_param_of_organization: ParamType,
    name_param_of_organization: String,
}

impl<S> OrganizationPermissionMiddleware<S> {
    pub fn new(
        permission_name: String,
        type_param_of_organization: ParamType,
        name_param_of_organization: String,
    ) -> Self {
        OrganizationPermissionMiddleware {
            _service: PhantomData,
            permission_name,
            type_param_of_organization,
            name_param_of_organization,
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for OrganizationPermissionMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = OrganizationPermissionMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ready(Ok(OrganizationPermissionMiddlewareService {
            service,
            permission_name: self.permission_name.clone(),
            type_param_of_organization: self.type_param_of_organization,
            name_param_of_organization: self.name_param_of_organization.clone(),
        }))
    }
}

pub struct OrganizationPermissionMiddlewareService<S> {
    service: S,
    permission_name: String,
    type_param_of_organization: ParamType,
    name_param_of_organization: String,
}

impl<S, B> Service<ServiceRequest> for OrganizationPermissionMiddlewareService<S>
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
        let permission_name = self.permission_name.clone();
        let name_param_of_organization = self.name_param_of_organization.clone();
        let type_param_of_organization = self.type_param_of_organization;

        let db_pool = match req.app_data::<web::Data<DbPool>>() {
            Some(pool) => pool.clone(),
            None => {
                let error = actix_web::error::ErrorInternalServerError("Failed to access database pool");
                return future::ready(Err(error)).boxed_local();
            },
        };

    let organization_id_str_opt = extract_param(&req, &name_param_of_organization, type_param_of_organization);
    // Capture UserJWT from request extensions before moving `req`
    let user_jwt_opt = req.extensions().get::<UserJWT>().cloned();

        let fut = self.service.call(req);

        async move {
            let organization_id = match organization_id_str_opt {
                Some(id_str) => id_str.parse::<i32>().ok(),
                None => None,
            }.ok_or(actix_web::error::ErrorBadRequest("Invalid or missing organization parameter"))?;

            let mut conn = match db_pool.get() {
                Ok(conn) => conn,
                Err(_) => return Err(actix_web::error::ErrorInternalServerError("Failed to get database connection")),
            };
            let user_jwt = match user_jwt_opt {
                Some(u) => u,
                None => return Err(actix_web::error::ErrorUnauthorized("Unauthorized access")),
            };

            match user_permission_organization_request(&mut conn, user_jwt.user_id, organization_id, &permission_name) {
                Ok(has_permission) => {
                    if !has_permission {
                        return Err(actix_web::error::ErrorForbidden("User does not have the required permission within the organization"));
                    }
                },
                Err(_) => return Err(actix_web::error::ErrorInternalServerError("Failed to check user permission within organization")),
            }

            fut.await
        }.boxed_local()
    }
}
