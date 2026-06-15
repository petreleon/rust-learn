use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::application::learning::learner_progress::{
    LearnerProgressError, LearnerProgressOutput, LearnerProgressStore, ProgressCourse,
    SaveLearnerProgressCommand,
};
use crate::domain::access_control::permissions::Permissions;
use crate::domain::access_control::roles::Roles;
use crate::domain::learning::course::status::COURSE_STATUS_PUBLISHED;
use crate::domain::learning::enrollment::status::COURSE_JOIN_STATUS_APPROVED;

pub async fn save_learner_progress(
    store: &mut impl LearnerProgressStore,
    command: SaveLearnerProgressCommand,
) -> Result<LearnerProgressOutput, LearnerProgressError> {
    ensure_progress_access(
        store,
        command.actor_user_id,
        command.course_id,
        Some(command.content_id),
    )
    .await?;

    store
        .save_progress(command.actor_user_id, command.course_id, command.content_id)
        .await
}

pub async fn get_learner_progress(
    store: &mut impl LearnerProgressStore,
    actor_user_id: i32,
    course_id: i32,
) -> Result<Option<LearnerProgressOutput>, LearnerProgressError> {
    ensure_progress_access(store, actor_user_id, course_id, None).await?;
    store.get_progress(actor_user_id, course_id).await
}

async fn ensure_progress_access(
    store: &mut impl LearnerProgressStore,
    actor_user_id: i32,
    course_id: i32,
    content_id: Option<i32>,
) -> Result<(), LearnerProgressError> {
    let course = store.course(course_id).await?;
    if !course_visible_to_learner(store, actor_user_id, &course).await? {
        return Err(LearnerProgressError::NotFound);
    }
    if !learner_has_enrolled_course_state(store, actor_user_id, course_id).await? {
        return Err(LearnerProgressError::PermissionDenied(
            Permissions::VIEW_CONTENT.into(),
        ));
    }
    if let Some(content_id) = content_id {
        ensure_content_belongs_to_course(store, course_id, content_id).await?;
    }
    Ok(())
}

async fn course_visible_to_learner(
    store: &mut impl LearnerProgressStore,
    actor_user_id: i32,
    course: &ProgressCourse,
) -> Result<bool, LearnerProgressError> {
    if course.lifecycle_status == COURSE_STATUS_PUBLISHED {
        return Ok(true);
    }
    if store
        .can(
            AccessActor::user(actor_user_id),
            AccessAction::permission(Permissions::VIEW_COURSE),
            AccessScope::course(course.id),
        )
        .await?
    {
        return Ok(true);
    }
    for organization_id in store.course_organization_ids(course.id).await? {
        if store
            .can(
                AccessActor::user(actor_user_id),
                AccessAction::permission(Permissions::VIEW_COURSE),
                AccessScope::organization(organization_id),
            )
            .await?
        {
            return Ok(true);
        }
    }
    Ok(false)
}

async fn learner_has_enrolled_course_state(
    store: &mut impl LearnerProgressStore,
    actor_user_id: i32,
    course_id: i32,
) -> Result<bool, LearnerProgressError> {
    let roles = store.actor_course_roles(actor_user_id, course_id).await?;
    if roles.iter().any(|role| role == &Roles::STUDENT.to_string()) {
        return Ok(true);
    }
    store
        .has_join_request_status(actor_user_id, course_id, COURSE_JOIN_STATUS_APPROVED)
        .await
}

async fn ensure_content_belongs_to_course(
    store: &mut impl LearnerProgressStore,
    course_id: i32,
    content_id: i32,
) -> Result<(), LearnerProgressError> {
    if store
        .content_belongs_to_course(course_id, content_id)
        .await?
    {
        Ok(())
    } else {
        Err(LearnerProgressError::NotFound)
    }
}
