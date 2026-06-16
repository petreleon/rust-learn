use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::application::learning::manage_assessments::{
    validation::validate_draft, AssessmentAuthoringCommand, AssessmentAuthoringError,
    AssessmentAuthoringStore, AssessmentUpdateCommand, AuthoredAssessmentOutput,
};
use crate::domain::access_control::permissions::Permissions;

pub async fn list_course_assessments_for_authoring(
    store: &mut impl AssessmentAuthoringStore,
    actor_user_id: i32,
    course_id: i32,
) -> Result<Vec<AuthoredAssessmentOutput>, AssessmentAuthoringError> {
    ensure_can_view_authoring(store, actor_user_id, course_id).await?;
    store.list_for_course(course_id).await
}

pub async fn create_assessment(
    store: &mut impl AssessmentAuthoringStore,
    command: AssessmentAuthoringCommand,
) -> Result<AuthoredAssessmentOutput, AssessmentAuthoringError> {
    ensure_can_create(store, command.actor_user_id, command.course_id).await?;
    if command.published {
        ensure_can_publish(store, command.actor_user_id, command.course_id).await?;
    }
    let course_id = command.course_id;
    let draft = command.into();
    validate_draft(&draft)?;
    store.create_assessment(course_id, draft).await
}

pub async fn update_assessment(
    store: &mut impl AssessmentAuthoringStore,
    command: AssessmentUpdateCommand,
) -> Result<AuthoredAssessmentOutput, AssessmentAuthoringError> {
    ensure_can_update(store, command.actor_user_id, command.course_id).await?;
    if command.published {
        ensure_can_publish(store, command.actor_user_id, command.course_id).await?;
    }
    let course_id = command.course_id;
    let assessment_id = command.assessment_id;
    let draft = command.into();
    validate_draft(&draft)?;
    store
        .update_assessment(course_id, assessment_id, draft)
        .await
}

async fn ensure_can_view_authoring(
    store: &mut impl AssessmentAuthoringStore,
    user_id: i32,
    course_id: i32,
) -> Result<(), AssessmentAuthoringError> {
    ensure_any_permission(
        store,
        user_id,
        course_id,
        &[
            Permissions::VIEW_ASSESSMENT,
            Permissions::CREATE_ASSESSMENT,
            Permissions::MODIFY_ASSESSMENT,
            Permissions::CREATE_CONTENT,
            Permissions::MODIFY_CONTENT,
            Permissions::MANAGE_COURSE_SETTINGS,
        ],
    )
    .await
}

async fn ensure_can_create(
    store: &mut impl AssessmentAuthoringStore,
    user_id: i32,
    course_id: i32,
) -> Result<(), AssessmentAuthoringError> {
    ensure_any_permission(
        store,
        user_id,
        course_id,
        &[
            Permissions::CREATE_ASSESSMENT,
            Permissions::CREATE_CONTENT,
            Permissions::MANAGE_COURSE_SETTINGS,
        ],
    )
    .await
}

async fn ensure_can_update(
    store: &mut impl AssessmentAuthoringStore,
    user_id: i32,
    course_id: i32,
) -> Result<(), AssessmentAuthoringError> {
    ensure_any_permission(
        store,
        user_id,
        course_id,
        &[
            Permissions::MODIFY_ASSESSMENT,
            Permissions::MODIFY_CONTENT,
            Permissions::MANAGE_COURSE_SETTINGS,
        ],
    )
    .await
}

async fn ensure_can_publish(
    store: &mut impl AssessmentAuthoringStore,
    user_id: i32,
    course_id: i32,
) -> Result<(), AssessmentAuthoringError> {
    ensure_any_permission(
        store,
        user_id,
        course_id,
        &[Permissions::PUBLISH_CONTENT, Permissions::MODIFY_ASSESSMENT],
    )
    .await
}

async fn ensure_any_permission(
    store: &mut impl AssessmentAuthoringStore,
    user_id: i32,
    course_id: i32,
    permissions: &[Permissions],
) -> Result<(), AssessmentAuthoringError> {
    let actor = AccessActor::user(user_id);
    for permission in permissions {
        if store
            .can(
                actor,
                AccessAction::permission(*permission),
                AccessScope::course(course_id),
            )
            .await?
        {
            return Ok(());
        }
    }
    Err(AssessmentAuthoringError::PermissionDenied(
        permissions[0].into(),
    ))
}
