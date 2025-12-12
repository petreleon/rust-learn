use actix_web::{dev::ServiceRequest, Error};
use diesel::PgConnection;

use crate::models::param_type::ParamType;
use crate::utils::{request_utils::extract_param, db_utils::course::user_permission_course_request};
use super::permission_middleware::{PermissionMiddleware, PermissionCheckStrategy};

#[derive(Clone)]
pub struct CourseStrategy {
    permission_name: String,
    type_param_of_course: ParamType,
    name_param_of_course: String,
}

pub struct CourseExtractedData {
    course_id: i32,
    permission_name: String,
}

// Implement Clone for ExtractedData to satisfy the trait bound (though it's simple data)
impl Clone for CourseExtractedData {
    fn clone(&self) -> Self {
        CourseExtractedData {
            course_id: self.course_id,
            permission_name: self.permission_name.clone(),
        }
    }
}

impl PermissionCheckStrategy for CourseStrategy {
    type ExtractedData = CourseExtractedData;

    fn extract(&self, req: &ServiceRequest) -> Result<Self::ExtractedData, Error> {
        let course_id_str_opt = extract_param(req, &self.name_param_of_course, self.type_param_of_course);
        
        let course_id = match course_id_str_opt {
             Some(id_str) => id_str.parse::<i32>().map_err(|_| actix_web::error::ErrorBadRequest("Invalid course ID format"))?,
             None => return Err(actix_web::error::ErrorBadRequest("Missing course parameter")),
        };

        Ok(CourseExtractedData {
            course_id,
            permission_name: self.permission_name.clone(),
        })
    }

    fn check(&self, pool: crate::db::DbPool, user_id: i32, data: Self::ExtractedData) -> futures::future::LocalBoxFuture<'static, Result<(), Error>> {
        use futures::FutureExt;

        let permission_name = data.permission_name.clone();

        async move {
            let mut conn = pool.get().await.map_err(|_| actix_web::error::ErrorInternalServerError("Failed to get database connection"))?;
            
            match user_permission_course_request(&mut conn, user_id, data.course_id, &permission_name).await {
                Ok(has_permission) => {
                    if !has_permission {
                        return Err(actix_web::error::ErrorForbidden("User does not have the required permission within the course"));
                    }
                    Ok(())
                },
                Err(_) => Err(actix_web::error::ErrorInternalServerError("Failed to check user permission within course")),
            }
        }.boxed_local()
    }
}

// Type alias for easier usage
pub type CoursePermissionMiddleware<S> = PermissionMiddleware<S, CourseStrategy>;

impl<S> CoursePermissionMiddleware<S> {
    pub fn new(
        permission_name: String,
        type_param_of_course: ParamType,
        name_param_of_course: String,
    ) -> Self {
        PermissionMiddleware::from_strategy(CourseStrategy {
            permission_name,
            type_param_of_course,
            name_param_of_course,
        })
    }
}
