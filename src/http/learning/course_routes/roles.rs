use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::learning::assign_course_role::{
    CourseRoleAssignmentError, CourseRoleAssignmentUseCase,
};
use crate::application::notifications::delivery::{
    NotificationDeliveryUseCase, RoleAssignmentNotification, RoleAssignmentScope,
};
use crate::http::extractors::auth_user::AuthUserId;
use crate::http::learning::dto::AssignCourseRoleRequest;

pub(super) async fn assign_role(
    requester: AuthUserId,
    path: web::Path<(i32, i32)>,
    body: web::Json<AssignCourseRoleRequest>,
    use_case: web::Data<Arc<dyn CourseRoleAssignmentUseCase>>,
    notifications: Option<web::Data<Arc<dyn NotificationDeliveryUseCase>>>,
) -> impl Responder {
    let (course_id, target_user_id) = path.into_inner();
    let requester_id = requester.into_inner();
    let command = body
        .into_inner()
        .into_command(requester_id, course_id, target_user_id);

    match use_case.assign_course_role(command).await {
        Ok(output) => {
            if let Some(notifications) = notifications {
                if let Err(err) = notifications
                    .send_role_assignment(RoleAssignmentNotification {
                        target_user_id: output.target_user_id,
                        scope: RoleAssignmentScope::Course,
                        scope_id: output.course_id,
                        role_name: output.role_name.clone(),
                    })
                    .await
                {
                    log::warn!(
                        "event=notification_send_failed kind=role_assignment scope=course course_id={} target_user_id={} error={}",
                        output.course_id,
                        output.target_user_id,
                        err.message()
                    );
                }
            }

            HttpResponse::Ok().body("Role assigned successfully")
        }
        Err(error) => course_role_assignment_error_response(error),
    }
}

fn course_role_assignment_error_response(error: CourseRoleAssignmentError) -> HttpResponse {
    match error {
        CourseRoleAssignmentError::HierarchyViolation => HttpResponse::Forbidden().body("Hierarchy check failed: Cannot assign role higher than or equal to your own, or modify user with higher/equal rank."),
        CourseRoleAssignmentError::NotFound => HttpResponse::BadRequest().body("Role or User not found"),
        CourseRoleAssignmentError::Connection(message) => {
            log::error!("event=course_role_assign_connection_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        CourseRoleAssignmentError::Database(message) => {
            log::error!("event=course_role_assign_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to assign role")
        }
    }
}
