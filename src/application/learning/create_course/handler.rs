use crate::application::learning::create_course::{
    CourseCreationCommand, CourseCreationError, CourseCreationOutput, CourseCreationStore,
};

const CREATE_COURSE: &str = "CREATE_COURSE";

pub async fn create_course(
    store: &mut impl CourseCreationStore,
    command: CourseCreationCommand,
) -> Result<CourseCreationOutput, CourseCreationError> {
    if !store
        .has_platform_permission(command.actor_user_id, CREATE_COURSE)
        .await?
        && !has_owner_organization_permission(store, &command).await?
    {
        return Err(CourseCreationError::PermissionDenied(
            CREATE_COURSE.to_string(),
        ));
    }

    store
        .create_course(command.title, command.organization_ids)
        .await
}

async fn has_owner_organization_permission(
    store: &mut impl CourseCreationStore,
    command: &CourseCreationCommand,
) -> Result<bool, CourseCreationError> {
    let Some(owner_organization_id) = command.organization_ids.first() else {
        return Ok(false);
    };

    store
        .has_organization_permission(command.actor_user_id, *owner_organization_id, CREATE_COURSE)
        .await
}
