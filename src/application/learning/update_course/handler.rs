use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::application::learning::update_course::{
    CourseUpdateCommand, CourseUpdateError, CourseUpdateOutput, CourseUpdateStore,
};

const MANAGE_COURSE_SETTINGS: &str = "MANAGE_COURSE_SETTINGS";
const MODIFY_COURSE: &str = "MODIFY_COURSE";

pub async fn update_course(
    store: &mut impl CourseUpdateStore,
    command: CourseUpdateCommand,
) -> Result<CourseUpdateOutput, CourseUpdateError> {
    let actor = AccessActor::user(command.actor_user_id);
    if !store
        .can(
            actor,
            AccessAction::permission(MANAGE_COURSE_SETTINGS),
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
        return Err(CourseUpdateError::PermissionDenied(
            MANAGE_COURSE_SETTINGS.to_string(),
        ));
    }

    store
        .update_course(command.course_id, command.patch())
        .await
}
