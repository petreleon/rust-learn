use actix_web::{dev::ServiceRequest, web, HttpMessage};
use futures::FutureExt;

use crate::models::param_type::ParamType;
use crate::utils::request_utils::extract_param;
use crate::repositories::organization_repository::user_permission_organization_request;
use crate::models::user_jwt::UserJWT;
use crate::middlewares::conditional_access_middleware::ConditionalAccessMiddleware;

pub struct OrganizationPermissionMiddleware;

impl OrganizationPermissionMiddleware {
    pub fn new<S>(
        permission_name: String,
        type_param_of_organization: ParamType,
        name_param_of_organization: String,
    ) -> ConditionalAccessMiddleware<S> {

        ConditionalAccessMiddleware::new(
            move |req: &ServiceRequest| {
                let permission_name = permission_name.clone();
                let type_param_of_organization = type_param_of_organization.clone();
                let name_param_of_organization = name_param_of_organization.clone();
                
                let db_pool = match req.app_data::<web::Data<crate::db::DbPool>>() {
                    Some(pool) => pool.get_ref().clone(),
                    None => return Box::pin(futures::future::ready(Err(actix_web::error::ErrorInternalServerError("Failed to access database pool")))),
                };

                let user_jwt = match req.extensions().get::<UserJWT>().cloned() {
                    Some(u) => u,
                    None => return Box::pin(futures::future::ready(Err(actix_web::error::ErrorUnauthorized("Unauthorized access")))),
                };

                let org_id_str_opt = extract_param(req, &name_param_of_organization, type_param_of_organization);
                let organization_id = match org_id_str_opt {
                    Some(id_str) => match id_str.parse::<i32>() {
                        Ok(id) => id,
                        Err(_) => return Box::pin(futures::future::ready(Err(actix_web::error::ErrorBadRequest("Invalid organization ID format")))),
                    },
                    None => return Box::pin(futures::future::ready(Err(actix_web::error::ErrorBadRequest("Missing organization parameter")))),
                };

                async move {
                    let mut conn = db_pool.get().await.map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get database connection"))?;
                    
                    match user_permission_organization_request(&mut conn, user_jwt.user_id, organization_id, &permission_name).await {
                        Ok(has_permission) => Ok(has_permission),
                        Err(_) => Err(actix_web::error::ErrorInternalServerError("Failed to check user permission within organization")),
                    }
                }.boxed_local()
            },
            || actix_web::error::ErrorForbidden("User does not have the required permission within the organization")
        )
    }
}
