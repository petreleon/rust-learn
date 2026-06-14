use actix_web::{dev::ServiceRequest, web, HttpMessage};
use futures::FutureExt;

use crate::application::access_control::check_permission::{
    PermissionCheckError, PermissionCheckService, PermissionScope,
};
use crate::domain::identity::UserJWT;
use crate::middlewares::conditional_access_middleware::ConditionalAccessMiddleware;

pub struct PlatformPermissionMiddleware;

impl PlatformPermissionMiddleware {
    pub fn require<S>(permission_name: String) -> ConditionalAccessMiddleware<S> {
        ConditionalAccessMiddleware::new(
            move |req: &ServiceRequest| {
                let permission_name = permission_name.clone();

                let permission_check = match req.app_data::<web::Data<PermissionCheckService>>() {
                    Some(pool) => pool.get_ref().clone(),
                    None => {
                        log::error!(
                            "event=permission_check_failed scope=platform reason=missing_permission_check_use_case permission={}",
                            permission_name
                        );
                        return Box::pin(futures::future::ready(Err(
                            actix_web::error::ErrorInternalServerError(
                                "Failed to access permission check use case",
                            ),
                        )));
                    }
                };

                let user_jwt = match req.extensions().get::<UserJWT>().cloned() {
                    Some(u) => u,
                    None => {
                        log::warn!(
                            "event=permission_denied scope=platform reason=missing_jwt permission={}",
                            permission_name
                        );
                        return Box::pin(futures::future::ready(Err(
                            actix_web::error::ErrorUnauthorized("Unauthorized access"),
                        )));
                    }
                };

                async move {
                    match permission_check
                        .has_permission(
                            user_jwt.user_id,
                            PermissionScope::Platform,
                            permission_name.clone(),
                        )
                        .await
                    {
                        Ok(true) => Ok(true),
                        Ok(false) => {
                            log::warn!(
                                "event=permission_denied scope=platform reason=missing_permission permission={} user_id={}",
                                permission_name,
                                user_jwt.user_id
                            );
                            Ok(false)
                        }
                        Err(PermissionCheckError::Connection(err)) => {
                            log::error!(
                                "event=permission_check_failed scope=platform reason=db_connection permission={} user_id={} error={}",
                                permission_name,
                                user_jwt.user_id,
                                err
                            );
                            Err(actix_web::error::ErrorInternalServerError(
                                "Failed to get database connection",
                            ))
                        }
                        Err(PermissionCheckError::Query(err)) => {
                            log::error!(
                                "event=permission_check_failed scope=platform reason=query permission={} user_id={} error={}",
                                permission_name,
                                user_jwt.user_id,
                                err
                            );
                            Err(actix_web::error::ErrorInternalServerError(
                                "Failed to check user permission",
                            ))
                        }
                    }
                }
                .boxed_local()
            },
            || actix_web::error::ErrorForbidden("User does not have the required permission"),
        )
    }
}
