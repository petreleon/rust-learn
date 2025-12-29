use actix_web::{dev::ServiceRequest, web, HttpMessage};
use futures::FutureExt;

use crate::repositories::platform_repository::user_permission_platform_request;
use crate::models::user_jwt::UserJWT;
use crate::middlewares::conditional_access_middleware::ConditionalAccessMiddleware;

pub struct PlatformPermissionMiddleware;

impl PlatformPermissionMiddleware {
    pub fn new<S>(permission_name: String) -> ConditionalAccessMiddleware<S> {
        ConditionalAccessMiddleware::new(
            move |req: &ServiceRequest| {
                let permission_name = permission_name.clone();
                
                let db_pool = match req.app_data::<web::Data<crate::db::DbPool>>() {
                    Some(pool) => pool.get_ref().clone(),
                    None => return Box::pin(futures::future::ready(Err(actix_web::error::ErrorInternalServerError("Failed to access database pool")))),
                };

                let user_jwt = match req.extensions().get::<UserJWT>().cloned() {
                    Some(u) => u,
                    None => return Box::pin(futures::future::ready(Err(actix_web::error::ErrorUnauthorized("Unauthorized access")))),
                };

                async move {
                    let mut conn = db_pool.get().await.map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get database connection"))?;
                    
                    match user_permission_platform_request(&mut conn, user_jwt.user_id, &permission_name).await {
                        Ok(has_permission) => Ok(has_permission),
                        Err(_) => Err(actix_web::error::ErrorInternalServerError("Failed to check user permission")),
                    }
                }.boxed_local()
            },
            || actix_web::error::ErrorForbidden("User does not have the required permission")
        )
    }
}
