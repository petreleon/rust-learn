use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::application::learning::update_course::{
    CourseUpdateCommand, CourseUpdateError, CourseUpdateOutput, CourseUpdateStore,
};
use crate::domain::access_control::permissions::Permissions;

pub async fn update_course(
    store: &mut impl CourseUpdateStore,
    command: CourseUpdateCommand,
) -> Result<CourseUpdateOutput, CourseUpdateError> {
    let actor = AccessActor::user(command.actor_user_id);
    if !store
        .can(
            actor,
            AccessAction::permission(Permissions::MANAGE_COURSE_SETTINGS),
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
        return Err(CourseUpdateError::PermissionDenied(
            Permissions::MANAGE_COURSE_SETTINGS.into(),
        ));
    }

    store
        .update_course(command.course_id, command.patch())
        .await
}
