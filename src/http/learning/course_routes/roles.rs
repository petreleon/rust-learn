use std::sync::Arc;

use actix_web::{http::StatusCode, web};

use crate::application::learning::assign_course_role::CourseRoleAssignmentUseCase;
use crate::application::notifications::delivery::{
    NotificationDeliveryUseCase, RoleAssignmentNotification, RoleAssignmentScope,
};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUserId;
use crate::http::learning::dto::AssignCourseRoleRequest;

use super::errors::course_role_assignment_error;

pub(super) async fn assign_role(
    requester: AuthUserId,
    path: web::Path<(i32, i32)>,
    body: web::Json<AssignCourseRoleRequest>,
    use_case: web::Data<Arc<dyn CourseRoleAssignmentUseCase>>,
    notifications: Option<web::Data<Arc<dyn NotificationDeliveryUseCase>>>,
) -> Result<(&'static str, StatusCode), ApiError> {
    let (course_id, target_user_id) = path.into_inner();
    let requester_id = requester.into_inner();
    let command = body
        .into_inner()
        .into_command(requester_id, course_id, target_user_id);

    let output = use_case
        .assign_course_role(command)
        .await
        .map_err(course_role_assignment_error)?;

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

    Ok(("Role assigned successfully", StatusCode::OK))
}
