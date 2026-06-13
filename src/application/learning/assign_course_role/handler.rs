use crate::application::learning::assign_course_role::{
    CourseRoleAssignmentCommand, CourseRoleAssignmentError, CourseRoleAssignmentOutput,
    CourseRoleAssignmentStore,
};

pub async fn assign_course_role(
    store: &mut impl CourseRoleAssignmentStore,
    command: CourseRoleAssignmentCommand,
) -> Result<CourseRoleAssignmentOutput, CourseRoleAssignmentError> {
    let actor_level = store
        .actor_min_level(command.actor_user_id, command.course_id)
        .await?
        .ok_or(CourseRoleAssignmentError::NotFound)?;
    let target_level = store
        .target_min_level(command.target_user_id, command.course_id)
        .await?;
    let role_id = store
        .role_id_by_name(&command.role_name)
        .await?
        .ok_or(CourseRoleAssignmentError::NotFound)?;
    let role_level = store
        .role_hierarchy_level(role_id)
        .await?
        .ok_or(CourseRoleAssignmentError::NotFound)?;

    if actor_level >= role_level || target_level.is_some_and(|level| actor_level >= level) {
        return Err(CourseRoleAssignmentError::HierarchyViolation);
    }

    store
        .assign_role(command.target_user_id, command.course_id, role_id)
        .await?;

    Ok(CourseRoleAssignmentOutput {
        course_id: command.course_id,
        target_user_id: command.target_user_id,
        role_name: command.role_name,
    })
}
