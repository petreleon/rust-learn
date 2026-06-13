use crate::application::learning::update_course_lifecycle::{
    CourseLifecycleCommand, CourseLifecycleError, CourseLifecycleOutput, CourseLifecycleStore,
};

const MANAGE_COURSE_SETTINGS: &str = "MANAGE_COURSE_SETTINGS";
const APPROVE_COURSE_CONTENT: &str = "APPROVE_COURSE_CONTENT";
const PUBLISH_CONTENT: &str = "PUBLISH_CONTENT";
const MODIFY_COURSE: &str = "MODIFY_COURSE";

pub async fn update_course_lifecycle(
    store: &mut impl CourseLifecycleStore,
    command: CourseLifecycleCommand,
) -> Result<CourseLifecycleOutput, CourseLifecycleError> {
    let target_status = normalize_course_status(&command.status)?;
    let course_permission = required_course_permission(&target_status)?;

    if !store
        .has_course_permission(command.actor_user_id, command.course_id, course_permission)
        .await?
        && !store
            .has_platform_permission(command.actor_user_id, MODIFY_COURSE)
            .await?
    {
        return Err(CourseLifecycleError::PermissionDenied(
            course_permission.to_string(),
        ));
    }

    store.update_status(command.course_id, target_status).await
}

fn required_course_permission(status: &str) -> Result<&'static str, CourseLifecycleError> {
    match status {
        "draft" | "submitted" | "archived" | "suspended" => Ok(MANAGE_COURSE_SETTINGS),
        "needs_changes" | "approved" => Ok(APPROVE_COURSE_CONTENT),
        "published" => Ok(PUBLISH_CONTENT),
        _ => Err(invalid_status_error()),
    }
}

fn normalize_course_status(status: &str) -> Result<String, CourseLifecycleError> {
    let normalized = status.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "draft" | "submitted" | "needs_changes" | "approved" | "published" | "archived"
        | "suspended" => Ok(normalized),
        _ => Err(invalid_status_error()),
    }
}

fn invalid_status_error() -> CourseLifecycleError {
    CourseLifecycleError::InvalidStatus("unsupported course lifecycle status".to_string())
}
