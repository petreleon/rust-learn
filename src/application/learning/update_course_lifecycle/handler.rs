use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::application::learning::update_course_lifecycle::{
    CourseLifecycleCommand, CourseLifecycleError, CourseLifecycleOutput, CourseLifecycleStore,
};
use crate::domain::learning::course::status::CourseLifecycleStatus;

const MANAGE_COURSE_SETTINGS: &str = "MANAGE_COURSE_SETTINGS";
const APPROVE_COURSE_CONTENT: &str = "APPROVE_COURSE_CONTENT";
const PUBLISH_CONTENT: &str = "PUBLISH_CONTENT";
const MODIFY_COURSE: &str = "MODIFY_COURSE";

pub async fn update_course_lifecycle(
    store: &mut impl CourseLifecycleStore,
    command: CourseLifecycleCommand,
) -> Result<CourseLifecycleOutput, CourseLifecycleError> {
    let target_status =
        CourseLifecycleStatus::normalize(&command.status).map_err(|_| invalid_status_error())?;
    let course_permission = required_course_permission(target_status);
    let actor = AccessActor::user(command.actor_user_id);

    if !store
        .can(
            actor,
            AccessAction::permission(course_permission),
            AccessScope::course(command.course_id),
        )
        .await?
        && !store
            .can(
                actor,
                AccessAction::permission(MODIFY_COURSE),
                AccessScope::platform(),
            )
            .await?
    {
        return Err(CourseLifecycleError::PermissionDenied(
            course_permission.to_string(),
        ));
    }

    store
        .update_status(command.course_id, target_status.as_str().to_string())
        .await
}

fn required_course_permission(status: CourseLifecycleStatus) -> &'static str {
    match status {
        CourseLifecycleStatus::Draft
        | CourseLifecycleStatus::Submitted
        | CourseLifecycleStatus::Archived
        | CourseLifecycleStatus::Suspended => MANAGE_COURSE_SETTINGS,
        CourseLifecycleStatus::NeedsChanges | CourseLifecycleStatus::Approved => {
            APPROVE_COURSE_CONTENT
        }
        CourseLifecycleStatus::Published => PUBLISH_CONTENT,
    }
}

fn invalid_status_error() -> CourseLifecycleError {
    CourseLifecycleError::InvalidStatus("unsupported course lifecycle status".to_string())
}
