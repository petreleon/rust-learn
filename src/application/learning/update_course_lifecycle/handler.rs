use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::application::learning::update_course_lifecycle::{
    CourseLifecycleCommand, CourseLifecycleError, CourseLifecycleOutput, CourseLifecycleStore,
};
use crate::domain::access_control::permissions::Permissions;
use crate::domain::learning::course::status::CourseLifecycleStatus;

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
                AccessAction::permission(Permissions::MODIFY_COURSE),
                AccessScope::platform(),
            )
            .await?
    {
        return Err(CourseLifecycleError::PermissionDenied(
            course_permission.into(),
        ));
    }

    let current_status = store.lifecycle_status(command.course_id).await?;
    let current_lifecycle =
        CourseLifecycleStatus::normalize(&current_status).map_err(|_| invalid_current_status())?;
    if !current_lifecycle.can_transition_to(target_status) {
        return Err(CourseLifecycleError::InvalidTransition(format!(
            "cannot transition course lifecycle from '{}' to '{}'",
            current_lifecycle.as_str(),
            target_status.as_str()
        )));
    }

    store
        .update_status(
            command.course_id,
            current_lifecycle.as_str().to_string(),
            target_status.as_str().to_string(),
        )
        .await
}

fn required_course_permission(status: CourseLifecycleStatus) -> Permissions {
    match status {
        CourseLifecycleStatus::Draft
        | CourseLifecycleStatus::Submitted
        | CourseLifecycleStatus::Archived
        | CourseLifecycleStatus::Suspended => Permissions::MANAGE_COURSE_SETTINGS,
        CourseLifecycleStatus::NeedsChanges | CourseLifecycleStatus::Approved => {
            Permissions::APPROVE_COURSE_CONTENT
        }
        CourseLifecycleStatus::Published => Permissions::PUBLISH_CONTENT,
    }
}

fn invalid_status_error() -> CourseLifecycleError {
    CourseLifecycleError::InvalidStatus("unsupported course lifecycle status".to_string())
}

fn invalid_current_status() -> CourseLifecycleError {
    CourseLifecycleError::InvalidStatus("course has unsupported lifecycle status".to_string())
}
