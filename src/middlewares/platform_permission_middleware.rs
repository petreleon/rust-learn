use actix_web::{dev::ServiceRequest, Error};
use diesel::PgConnection;

use crate::utils::db_utils::platform::user_permission_platform_request;
use super::permission_middleware::{PermissionMiddleware, PermissionCheckStrategy};

#[derive(Clone)]
pub struct PlatformStrategy {
    permission_name: String,
}

pub struct PlatformExtractedData {
    permission_name: String,
}

impl Clone for PlatformExtractedData {
    fn clone(&self) -> Self {
        PlatformExtractedData {
            permission_name: self.permission_name.clone(),
        }
    }
}

impl PermissionCheckStrategy for PlatformStrategy {
    type ExtractedData = PlatformExtractedData;

    fn extract(&self, _req: &ServiceRequest) -> Result<Self::ExtractedData, Error> {
        // Platform permissions don't need extracted ID currently
        Ok(PlatformExtractedData {
            permission_name: self.permission_name.clone(),
        })
    }

    fn check(&self, conn: &mut PgConnection, user_id: i32, data: Self::ExtractedData) -> Result<(), Error> {
        match user_permission_platform_request(conn, user_id, &data.permission_name) {
            Ok(has_permission) => {
                if !has_permission {
                    return Err(actix_web::error::ErrorForbidden("User does not have the required permission"));
                }
                Ok(())
            },
            Err(_) => Err(actix_web::error::ErrorInternalServerError("Failed to check user permission")),
        }
    }
}

pub type PlatformPermissionMiddleware<S> = PermissionMiddleware<S, PlatformStrategy>;

impl<S> PlatformPermissionMiddleware<S> {
    pub fn new(permission_name: String) -> Self {
        PermissionMiddleware::from_strategy(PlatformStrategy {
            permission_name,
        })
    }
}
