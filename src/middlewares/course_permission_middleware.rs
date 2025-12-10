use actix_service::{forward_ready, Service, Transform};
use actix_web::{dev::{ServiceRequest, ServiceResponse}, Error, web, HttpMessage};
use futures::future::{self, Ready, LocalBoxFuture};
use futures::FutureExt;
use std::marker::PhantomData;

use crate::db::DbPool;
use crate::models::{user_jwt::UserJWT, param_type::ParamType};
use crate::utils::{request_utils::extract_param, db_utils::course::user_permission_course_request};

pub struct CoursePermissionMiddleware<S> {
    _service: PhantomData<S>,
    permission_name: String,
    type_param_of_course: ParamType,
    name_param_of_course: String,
}

impl<S> CoursePermissionMiddleware<S> {
    pub fn new(
        permission_name: String,
        type_param_of_course: ParamType,
        name_param_of_course: String,
    ) -> Self {
        CoursePermissionMiddleware {
            _service: PhantomData,
            permission_name,
            type_param_of_course,
            name_param_of_course,
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for CoursePermissionMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CoursePermissionMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ready(Ok(CoursePermissionMiddlewareService {
            service,
            permission_name: self.permission_name.clone(),
            type_param_of_course: self.type_param_of_course,
            name_param_of_course: self.name_param_of_course.clone(),
        }))
    }
}

pub struct CoursePermissionMiddlewareService<S> {
    service: S,
    permission_name: String,
    type_param_of_course: ParamType,
    name_param_of_course: String,
}

impl<S, B> Service<ServiceRequest> for CoursePermissionMiddlewareService<S>
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
        let name_param_of_course = self.name_param_of_course.clone();
        let type_param_of_course = self.type_param_of_course;

        let db_pool = match req.app_data::<web::Data<DbPool>>() {
            Some(pool) => pool.clone(),
            None => {
                let error = actix_web::error::ErrorInternalServerError("Failed to access database pool");
                return future::ready(Err(error)).boxed_local();
            },
        };

        let course_id_str_opt = extract_param(&req, &name_param_of_course, type_param_of_course);
        // Capture UserJWT from request extensions before moving `req`
        let user_jwt_opt = req.extensions().get::<UserJWT>().cloned();

        let fut = self.service.call(req);

        async move {
            let course_id = match course_id_str_opt {
                Some(id_str) => id_str.parse::<i32>().ok(),
                None => None,
            }.ok_or(actix_web::error::ErrorBadRequest("Invalid or missing course parameter"))?;

            let mut conn = match db_pool.get() {
                Ok(conn) => conn,
                Err(_) => return Err(actix_web::error::ErrorInternalServerError("Failed to get database connection")),
            };
            let user_jwt = match user_jwt_opt {
                Some(u) => u,
                None => return Err(actix_web::error::ErrorUnauthorized("Unauthorized access")),
            };

            match user_permission_course_request(&mut conn, user_jwt.user_id, course_id, &permission_name) {
                Ok(has_permission) => {
                    if !has_permission {
                        return Err(actix_web::error::ErrorForbidden("User does not have the required permission within the course"));
                    }
                },
                Err(_) => return Err(actix_web::error::ErrorInternalServerError("Failed to check user permission within course")),
            }

            fut.await
        }.boxed_local()
    }
}
