use actix_web::{dev::ServiceRequest, Error};
use diesel::PgConnection;

use crate::models::param_type::ParamType;
use crate::utils::{request_utils::extract_param, db_utils::organization::user_permission_organization_request};
use super::permission_middleware::{PermissionMiddleware, PermissionCheckStrategy};

#[derive(Clone)]
pub struct OrganizationStrategy {
    permission_name: String,
    type_param_of_organization: ParamType,
    name_param_of_organization: String,
}

pub struct OrganizationExtractedData {
    organization_id: i32,
    permission_name: String,
}

impl Clone for OrganizationExtractedData {
    fn clone(&self) -> Self {
        OrganizationExtractedData {
            organization_id: self.organization_id,
            permission_name: self.permission_name.clone(),
        }
    }
}

impl PermissionCheckStrategy for OrganizationStrategy {
    type ExtractedData = OrganizationExtractedData;

    fn extract(&self, req: &ServiceRequest) -> Result<Self::ExtractedData, Error> {
        let org_id_str_opt = extract_param(req, &self.name_param_of_organization, self.type_param_of_organization);
        
        let organization_id = match org_id_str_opt {
             Some(id_str) => id_str.parse::<i32>().map_err(|_| actix_web::error::ErrorBadRequest("Invalid organization ID format"))?,
             None => return Err(actix_web::error::ErrorBadRequest("Missing organization parameter")),
        };

        Ok(OrganizationExtractedData {
            organization_id,
            permission_name: self.permission_name.clone(),
        })
    }

    fn check(&self, pool: crate::db::DbPool, user_id: i32, data: Self::ExtractedData) -> futures::future::LocalBoxFuture<'static, Result<(), Error>> {
        use futures::FutureExt;
        
        let permission_name = data.permission_name.clone();
        
        async move {
            let mut conn = pool.get().await.map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get database connection"))?;
            
            match user_permission_organization_request(&mut conn, user_id, data.organization_id, &permission_name).await {
                Ok(has_permission) => {
                    if !has_permission {
                        return Err(actix_web::error::ErrorForbidden("User does not have the required permission within the organization"));
                    }
                    Ok(())
                },
                Err(_) => Err(actix_web::error::ErrorInternalServerError("Failed to check user permission within organization")),
            }
        }.boxed_local()
    }
}

pub type OrganizationPermissionMiddleware<S> = PermissionMiddleware<S, OrganizationStrategy>;

impl<S> OrganizationPermissionMiddleware<S> {
    pub fn new(
        permission_name: String,
        type_param_of_organization: ParamType,
        name_param_of_organization: String,
    ) -> Self {
        PermissionMiddleware::from_strategy(OrganizationStrategy {
            permission_name,
            type_param_of_organization,
            name_param_of_organization,
        })
    }
}
