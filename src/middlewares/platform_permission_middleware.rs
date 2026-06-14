use actix_web::{dev::ServiceRequest, web, HttpMessage};
use futures::FutureExt;

use crate::infra::postgres::access_control::authorization_checks::user_permission_platform_request;
use crate::middlewares::conditional_access_middleware::ConditionalAccessMiddleware;
use crate::models::user_jwt::UserJWT;

pub struct PlatformPermissionMiddleware;

impl PlatformPermissionMiddleware {
    pub fn require<S>(permission_name: String) -> ConditionalAccessMiddleware<S> {
        ConditionalAccessMiddleware::new(
            move |req: &ServiceRequest| {
                let permission_name = permission_name.clone();

                let db_pool = match req.app_data::<web::Data<crate::db::DbPool>>() {
                    Some(pool) => pool.get_ref().clone(),
                    None => {
                        log::error!(
                            "event=permission_check_failed scope=platform reason=missing_db_pool permission={}",
                            permission_name
                        );
                        return Box::pin(futures::future::ready(Err(
                            actix_web::error::ErrorInternalServerError(
                                "Failed to access database pool",
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
                    let mut conn = db_pool.get().await.map_err(|_| {
                        log::error!(
                            "event=permission_check_failed scope=platform reason=db_connection permission={} user_id={}",
                            permission_name,
                            user_jwt.user_id
                        );
                        actix_web::error::ErrorInternalServerError(
                            "Failed to get database connection",
                        )
                    })?;

                    match user_permission_platform_request(
                        &mut conn,
                        user_jwt.user_id,
                        &permission_name,
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
                        Err(err) => {
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
