use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionStore, AccessScope,
};
use crate::application::teacher_applications::list_platform_review::{
    TeacherApplicationPlatformReviewError, TeacherApplicationPlatformReviewItemOutput,
    TeacherApplicationPlatformReviewOutput, TeacherApplicationPlatformReviewPermissionsOutput,
    TeacherApplicationPlatformReviewQuery, TeacherApplicationPlatformReviewStore,
};
use crate::domain::access_control::permissions::Permissions;
use crate::domain::teacher_applications::status::normalize_optional_status;

pub async fn list_platform_review_applications(
    store: &mut impl TeacherApplicationPlatformReviewStore,
    query: TeacherApplicationPlatformReviewQuery,
) -> Result<TeacherApplicationPlatformReviewOutput, TeacherApplicationPlatformReviewError> {
    if !can_platform_action(
        store,
        query.actor_user_id,
        Permissions::REVIEW_TEACHER_APPLICATIONS,
    )
    .await?
    {
        return Err(TeacherApplicationPlatformReviewError::PermissionDenied(
            Permissions::REVIEW_TEACHER_APPLICATIONS.into(),
        ));
    }

    let status = normalize_optional_status(query.status)?;
    let search = normalize_optional_text(query.search);
    let limit = query.limit.unwrap_or(25).clamp(1, 100);
    let offset = query.offset.unwrap_or(0).max(0);
    let can_approve_applications = can_platform_action(
        store,
        query.actor_user_id,
        Permissions::APPROVE_TEACHER_APPLICATION,
    )
    .await?;
    let can_reject_applications = can_platform_action(
        store,
        query.actor_user_id,
        Permissions::REJECT_TEACHER_APPLICATION,
    )
    .await?;
    let dataset = store.list_applications().await?;
    let mut applications = dataset.applications;

    if let Some(status) = status.as_deref() {
        applications.retain(|application| application.status == status);
    }
    if let Some(search) = search.as_deref() {
        let normalized = search.to_lowercase();
        applications.retain(|application| application_matches_search(application, &normalized));
    }

    let total = applications.len() as i64;
    let applications = applications
        .into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .collect::<Vec<_>>();

    Ok(TeacherApplicationPlatformReviewOutput {
        applications,
        summary: dataset.summary,
        operator_permissions: TeacherApplicationPlatformReviewPermissionsOutput {
            can_view_applications: true,
            can_approve_applications,
            can_reject_applications,
            can_request_changes: true,
        },
        total,
        limit,
        offset,
        status,
        search,
    })
}

async fn can_platform_action(
    store: &mut impl AccessDecisionStore<Error = TeacherApplicationPlatformReviewError>,
    actor_user_id: i32,
    permission: Permissions,
) -> Result<bool, TeacherApplicationPlatformReviewError> {
    store
        .can(
            AccessActor::user(actor_user_id),
            AccessAction::permission(permission),
            AccessScope::platform(),
        )
        .await
}

fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|text| {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

fn application_matches_search(
    application: &TeacherApplicationPlatformReviewItemOutput,
    search: &str,
) -> bool {
    application.id.to_string().contains(search)
        || application.applicant.id.to_string().contains(search)
        || application.applicant.name.to_lowercase().contains(search)
        || application.applicant.email.to_lowercase().contains(search)
        || application
            .experience_summary
            .to_lowercase()
            .contains(search)
        || application.status.to_lowercase().contains(search)
        || application.requested_scope.to_lowercase().contains(search)
        || application.requested_course.as_ref().is_some_and(|course| {
            course.id.to_string().contains(search) || course.title.to_lowercase().contains(search)
        })
        || application
            .requested_organization
            .as_ref()
            .is_some_and(|organization| {
                organization.id.to_string().contains(search)
                    || organization.name.to_lowercase().contains(search)
            })
        || application
            .sponsor_organization
            .as_ref()
            .is_some_and(|organization| {
                organization.id.to_string().contains(search)
                    || organization.name.to_lowercase().contains(search)
            })
}

#[cfg(test)]
mod tests;
