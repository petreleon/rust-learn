use bigdecimal::BigDecimal;

use crate::application::learning::manage_course_completion_terms::{
    CourseCompletionTermsError, CourseCompletionTermsOutput, CourseCompletionTermsStore,
};
use crate::domain::access_control::permissions::Permissions;
use crate::domain::learning::course::CourseCompletionTermsStatus;

pub(super) fn validate_terms_values(
    reward_amount: &BigDecimal,
    max_enrolled_students: i32,
) -> Result<(), CourseCompletionTermsError> {
    if reward_amount < &BigDecimal::from(0) {
        return Err(CourseCompletionTermsError::InvalidInput(
            "completion reward amount cannot be negative".to_string(),
        ));
    }
    if max_enrolled_students <= 0 {
        return Err(CourseCompletionTermsError::InvalidInput(
            "max enrolled students must be greater than zero".to_string(),
        ));
    }
    Ok(())
}

pub(super) fn ensure_open_terms(
    terms: &CourseCompletionTermsOutput,
) -> Result<(), CourseCompletionTermsError> {
    if matches!(
        terms.status,
        CourseCompletionTermsStatus::Submitted | CourseCompletionTermsStatus::Countered
    ) {
        Ok(())
    } else {
        Err(CourseCompletionTermsError::InvalidStatus(
            "course completion terms are not open for negotiation".to_string(),
        ))
    }
}

pub(super) async fn ensure_can_view(
    store: &mut impl CourseCompletionTermsStore,
    actor_user_id: i32,
    course_id: i32,
) -> Result<(), CourseCompletionTermsError> {
    ensure_any_permission(
        store,
        actor_user_id,
        course_id,
        &[
            Permissions::MANAGE_COURSE_SETTINGS,
            Permissions::MANAGE_COURSE_REWARD_RULES,
            Permissions::VIEW_COURSE_REWARD_STATUS,
            Permissions::SET_REWARD_POLICY,
            Permissions::MANAGE_ORG_REWARD_BUDGET,
        ],
    )
    .await
}

pub(super) async fn ensure_can_propose(
    store: &mut impl CourseCompletionTermsStore,
    actor_user_id: i32,
    course_id: i32,
) -> Result<(), CourseCompletionTermsError> {
    ensure_any_permission(
        store,
        actor_user_id,
        course_id,
        &[
            Permissions::MANAGE_COURSE_SETTINGS,
            Permissions::MANAGE_COURSE_REWARD_RULES,
        ],
    )
    .await
}

pub(super) async fn ensure_can_decide(
    store: &mut impl CourseCompletionTermsStore,
    actor_user_id: i32,
    course_id: i32,
) -> Result<(), CourseCompletionTermsError> {
    ensure_any_permission(
        store,
        actor_user_id,
        course_id,
        &[
            Permissions::SET_REWARD_POLICY,
            Permissions::MANAGE_ORG_REWARD_BUDGET,
        ],
    )
    .await
}

async fn ensure_any_permission(
    store: &mut impl CourseCompletionTermsStore,
    actor_user_id: i32,
    course_id: i32,
    permissions: &[Permissions],
) -> Result<(), CourseCompletionTermsError> {
    for permission in permissions {
        let permission = permission.to_string();
        if store
            .has_course_permission(actor_user_id, course_id, &permission)
            .await?
        {
            return Ok(());
        }
    }
    Err(CourseCompletionTermsError::PermissionDenied(
        permissions[0].to_string(),
    ))
}
