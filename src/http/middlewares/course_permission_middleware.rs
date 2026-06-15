use actix_web::{dev::ServiceRequest, web, HttpMessage};
use futures::FutureExt;

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionError, AccessDecisionService, AccessScope,
};
use crate::domain::identity::UserJWT;
use crate::http::middlewares::conditional_access_middleware::ConditionalAccessMiddleware;
use crate::http::request_params::extract_param;
use crate::http::request_params::ParamType;

pub struct CoursePermissionMiddleware;

impl CoursePermissionMiddleware {
    pub fn require<S>(
        permission_name: String,
        type_param_of_course: ParamType,
        name_param_of_course: String,
    ) -> ConditionalAccessMiddleware<S> {
        ConditionalAccessMiddleware::new(
            move |req: &ServiceRequest| {
                let permission_name = permission_name.clone();
                let type_param_of_course = type_param_of_course;
                let name_param_of_course = name_param_of_course.clone();

                // Extract data synchronously (as much as possible that doesn't need async)
                // But we need to move it into the async block.

                let access_decision = match req.app_data::<web::Data<AccessDecisionService>>() {
                    Some(pool) => pool.get_ref().clone(),
                    None => {
                        log::error!(
                            "event=permission_check_failed scope=course reason=missing_access_decision_service permission={}",
                            permission_name
                        );
                        return Box::pin(futures::future::ready(Err(
                            actix_web::error::ErrorInternalServerError(
                                "Failed to access permission decision service",
                            ),
                        )));
                    }
                };

                // 2. Get UserJWT
                let user_jwt = match req.extensions().get::<UserJWT>().cloned() {
                    Some(u) => u,
                    None => {
                        log::warn!(
                            "event=permission_denied scope=course reason=missing_jwt permission={}",
                            permission_name
                        );
                        return Box::pin(futures::future::ready(Err(
                            actix_web::error::ErrorUnauthorized("Unauthorized access"),
                        )));
                    }
                };

                // 3. Extract Course ID
                let course_id_str_opt =
                    extract_param(req, &name_param_of_course, type_param_of_course);
                let course_id = match course_id_str_opt {
                    Some(id_str) => match id_str.parse::<i32>() {
                        Ok(id) => id,
                        Err(_) => {
                            log::warn!(
                                "event=permission_denied scope=course reason=invalid_scope_id permission={} raw_scope_id={}",
                                permission_name,
                                id_str
                            );
                            return Box::pin(futures::future::ready(Err(
                                actix_web::error::ErrorBadRequest("Invalid course ID format"),
                            )));
                        }
                    },
                    None => {
                        log::warn!(
                            "event=permission_denied scope=course reason=missing_scope_id permission={}",
                            permission_name
                        );
                        return Box::pin(futures::future::ready(Err(
                            actix_web::error::ErrorBadRequest("Missing course parameter"),
                        )));
                    }
                };

                async move {
                    match access_decision
                        .can(
                            AccessActor::user(user_jwt.user_id),
                            AccessAction::permission(permission_name.clone()),
                            AccessScope::course(course_id),
                        )
                        .await
                    {
                        Ok(true) => Ok(true),
                        Ok(false) => {
                            log::warn!(
                                "event=permission_denied scope=course reason=missing_permission permission={} user_id={} course_id={}",
                                permission_name,
                                user_jwt.user_id,
                                course_id
                            );
                            Ok(false)
                        }
                        Err(AccessDecisionError::Connection(err)) => {
                            log::error!(
                                "event=permission_check_failed scope=course reason=db_connection permission={} user_id={} course_id={} error={}",
                                permission_name,
                                user_jwt.user_id,
                                course_id,
                                err
                            );
                            Err(actix_web::error::ErrorInternalServerError(
                                "Failed to get database connection",
                            ))
                        }
                        Err(AccessDecisionError::Query(err)) => {
                            log::error!(
                                "event=permission_check_failed scope=course reason=query permission={} user_id={} course_id={} error={}",
                                permission_name,
                                user_jwt.user_id,
                                course_id,
                                err
                            );
                            Err(actix_web::error::ErrorInternalServerError(
                                "Failed to check user permission within course",
                            ))
                        }
                    }
                }
                .boxed_local()
            },
            || {
                actix_web::error::ErrorForbidden(
                    "User does not have the required permission within the course",
                )
            },
        )
    }
}
