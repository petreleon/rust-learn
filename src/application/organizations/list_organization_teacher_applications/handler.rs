use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionStore, AccessScope,
};
use crate::application::organizations::list_organization_teacher_applications::{
    OrganizationTeacherApplicationListError, OrganizationTeacherApplicationListOutput,
    OrganizationTeacherApplicationListQuery, OrganizationTeacherApplicationListStore,
};

const VIEW_ORG_TEACHER_APPLICATIONS: &str = "VIEW_ORG_TEACHER_APPLICATIONS";
const NOMINATE_TEACHER_FOR_PLATFORM_REVIEW: &str = "NOMINATE_TEACHER_FOR_PLATFORM_REVIEW";

pub async fn list_organization_teacher_applications(
    store: &mut impl OrganizationTeacherApplicationListStore,
    query: OrganizationTeacherApplicationListQuery,
) -> Result<OrganizationTeacherApplicationListOutput, OrganizationTeacherApplicationListError> {
    let organization = store.organization(query.organization_id).await?;
    let can_view_applications = can_platform_or_organization_action(
        store,
        query.actor_user_id,
        query.organization_id,
        VIEW_ORG_TEACHER_APPLICATIONS,
    )
    .await?;
    let can_nominate_teachers = can_platform_or_organization_action(
        store,
        query.actor_user_id,
        query.organization_id,
        NOMINATE_TEACHER_FOR_PLATFORM_REVIEW,
    )
    .await?;

    if !can_view_applications && !can_nominate_teachers {
        return Err(OrganizationTeacherApplicationListError::PermissionDenied(
            VIEW_ORG_TEACHER_APPLICATIONS.to_string(),
        ));
    }

    let status = normalize_status(query.status)?;
    let search = normalize_optional_text(query.search);
    let limit = query.limit.unwrap_or(25).clamp(1, 100);
    let offset = query.offset.unwrap_or(0).max(0);
    let dataset = store
        .list_teacher_applications(query.organization_id)
        .await?;
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

    Ok(OrganizationTeacherApplicationListOutput {
        organization,
        applications,
        summary: dataset.summary,
        operator_permissions: dataset
            .operator_permissions
            .with(can_view_applications, can_nominate_teachers),
        total,
        limit,
        offset,
        status,
        search,
    })
}

async fn can_platform_or_organization_action(
    store: &mut impl AccessDecisionStore<Error = OrganizationTeacherApplicationListError>,
    actor_user_id: i32,
    organization_id: i32,
    permission: &'static str,
) -> Result<bool, OrganizationTeacherApplicationListError> {
    let actor = AccessActor::user(actor_user_id);
    if store
        .can(
            actor,
            AccessAction::permission(permission),
            AccessScope::platform(),
        )
        .await?
    {
        return Ok(true);
    }
    store
        .can(
            actor,
            AccessAction::permission(permission),
            AccessScope::organization(organization_id),
        )
        .await
}

fn normalize_status(
    status: Option<String>,
) -> Result<Option<String>, OrganizationTeacherApplicationListError> {
    let Some(status) = status else {
        return Ok(None);
    };
    let normalized = status.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "submitted" | "needs_changes" | "approved" | "rejected" => Ok(Some(normalized)),
        _ => Err(OrganizationTeacherApplicationListError::InvalidInput(
            "unsupported teacher application status".to_string(),
        )),
    }
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
    application: &crate::application::organizations::list_organization_teacher_applications::OrganizationTeacherApplicationItemOutput,
    search: &str,
) -> bool {
    application.applicant.name.to_lowercase().contains(search)
        || application.applicant.email.to_lowercase().contains(search)
        || application
            .experience_summary
            .to_lowercase()
            .contains(search)
        || application.status.to_lowercase().contains(search)
        || application.requested_scope.to_lowercase().contains(search)
        || application
            .requested_course
            .as_ref()
            .is_some_and(|course| course.title.to_lowercase().contains(search))
        || application
            .requested_organization
            .as_ref()
            .is_some_and(|organization| organization.name.to_lowercase().contains(search))
}

#[cfg(test)]
mod tests;
