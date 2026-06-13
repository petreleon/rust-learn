use crate::application::learning::update_course::{
    CourseUpdateCommand, CourseUpdateError, CourseUpdateOutput, CourseUpdateStore,
};

const MANAGE_COURSE_SETTINGS: &str = "MANAGE_COURSE_SETTINGS";
const MODIFY_COURSE: &str = "MODIFY_COURSE";

pub async fn update_course(
    store: &mut impl CourseUpdateStore,
    command: CourseUpdateCommand,
) -> Result<CourseUpdateOutput, CourseUpdateError> {
    if !store
        .has_course_permission(
            command.actor_user_id,
            command.course_id,
            MANAGE_COURSE_SETTINGS,
        )
        .await?
        && !store
            .has_platform_permission(command.actor_user_id, MODIFY_COURSE)
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
