use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::application::teacher_applications::{
    nominate_application::{
        TeacherApplicationNominationCommand, TeacherApplicationNominationError,
        TeacherApplicationNominationStore,
    },
    submit_application::TeacherApplicationSubmission,
    TeacherApplicationOutput,
};
use crate::domain::access_control::permissions::Permissions;
use crate::domain::teacher_applications::{
    portfolio::portfolio_links_from_urls,
    scope::{normalize_scope, validate_requested_scope, TEACHER_APPLICATION_SCOPE_ORGANIZATION},
    status::{TEACHER_APPLICATION_STATUS_REJECTED, TEACHER_APPLICATION_STATUS_SUBMITTED},
};
pub async fn nominate_application(
    store: &mut impl TeacherApplicationNominationStore,
    command: TeacherApplicationNominationCommand,
) -> Result<TeacherApplicationOutput, TeacherApplicationNominationError> {
    let permission = Permissions::NOMINATE_TEACHER_FOR_PLATFORM_REVIEW;
    if !store
        .can(
            AccessActor::user(command.actor_user_id),
            AccessAction::permission(permission),
            AccessScope::organization(command.organization_id),
        )
        .await?
    {
        return Err(TeacherApplicationNominationError::PermissionDenied(
            permission.into(),
        ));
    }
    let submission = build_nomination_submission(&command)?;
    if let Some(existing) = find_idempotent_application(store, &submission).await? {
        log::info!(
            "event=teacher_application_idempotent_replay application_id={} actor_user_id={} applicant_user_id={} status={} requested_scope={} idempotency_key={}",
            existing.id,
            command.actor_user_id,
            existing.applicant_user_id,
            existing.status,
            existing.requested_scope,
            existing.idempotency_key.as_deref().unwrap_or("none")
        );
        return Ok(existing);
    }
    ensure_no_blocking_application(store, &submission).await?;

    let application = store
        .create_organization_nomination(command.actor_user_id, submission)
        .await?;
    log::info!(
        "event=teacher_application_transition application_id={} actor_user_id={} applicant_user_id={} transition=organization_nominated from_status=none to_status={} requested_scope={} organization_sponsor_id={:?} requested_organization_id={:?} requested_course_id={:?}",
        application.id,
        command.actor_user_id,
        application.applicant_user_id,
        application.status,
        application.requested_scope,
        application.organization_sponsor_id,
        application.requested_organization_id,
        application.requested_course_id
    );

    Ok(application)
}
fn build_nomination_submission(
    command: &TeacherApplicationNominationCommand,
) -> Result<TeacherApplicationSubmission, TeacherApplicationNominationError> {
    let requested_scope = command
        .requested_scope
        .as_deref()
        .unwrap_or(TEACHER_APPLICATION_SCOPE_ORGANIZATION);
    let requested_scope = normalize_scope(requested_scope)?;
    let experience_summary = command.experience_summary.trim().to_string();
    if experience_summary.is_empty() {
        return Err(TeacherApplicationNominationError::InvalidInput(
            "experience_summary is required".to_string(),
        ));
    }
    validate_requested_scope(
        &requested_scope,
        Some(command.organization_id),
        command.requested_course_id,
        Some(command.organization_id),
    )?;
    Ok(TeacherApplicationSubmission {
        applicant_user_id: command.applicant_user_id,
        requested_scope,
        requested_organization_id: Some(command.organization_id),
        requested_course_id: command.requested_course_id,
        experience_summary,
        organization_sponsor_id: Some(command.organization_id),
        portfolio_links: portfolio_links_from_urls(clean_portfolio_links(
            command.portfolio_links.clone(),
        )),
        idempotency_key: normalize_idempotency_key(command.idempotency_key.clone())?,
        status: TEACHER_APPLICATION_STATUS_SUBMITTED.to_string(),
    })
}

async fn find_idempotent_application(
    store: &mut impl TeacherApplicationNominationStore,
    submission: &TeacherApplicationSubmission,
) -> Result<Option<TeacherApplicationOutput>, TeacherApplicationNominationError> {
    let Some(idempotency_key) = submission.idempotency_key.as_ref() else {
        return Ok(None);
    };
    let existing = store
        .find_application_by_idempotency_key(idempotency_key.clone())
        .await?;
    if let Some(existing) = existing.as_ref() {
        ensure_idempotent_application_matches(existing, submission)?;
    }
    Ok(existing)
}
async fn ensure_no_blocking_application(
    store: &mut impl TeacherApplicationNominationStore,
    submission: &TeacherApplicationSubmission,
) -> Result<(), TeacherApplicationNominationError> {
    let existing = store
        .find_latest_application_for_applicant(submission.applicant_user_id)
        .await?;
    match existing {
        Some(application) if application.status != TEACHER_APPLICATION_STATUS_REJECTED => Err(
            TeacherApplicationNominationError::InvalidTransition(format!(
                "teacher application already exists with status {}",
                application.status
            )),
        ),
        _ => Ok(()),
    }
}
fn ensure_idempotent_application_matches(
    existing: &TeacherApplicationOutput,
    submission: &TeacherApplicationSubmission,
) -> Result<(), TeacherApplicationNominationError> {
    if existing.applicant_user_id == submission.applicant_user_id
        && existing.requested_scope == submission.requested_scope
        && existing.requested_organization_id == submission.requested_organization_id
        && existing.requested_course_id == submission.requested_course_id
        && existing.experience_summary == submission.experience_summary
        && existing.organization_sponsor_id == submission.organization_sponsor_id
        && existing.portfolio_links == submission.portfolio_links
    {
        Ok(())
    } else {
        Err(TeacherApplicationNominationError::InvalidInput(
            "teacher application idempotency key is already used by another application"
                .to_string(),
        ))
    }
}
fn clean_portfolio_links(portfolio_links: Option<Vec<String>>) -> Vec<String> {
    portfolio_links
        .unwrap_or_default()
        .into_iter()
        .map(|link| link.trim().to_string())
        .filter(|link| !link.is_empty())
        .collect()
}
fn normalize_idempotency_key(
    idempotency_key: Option<String>,
) -> Result<Option<String>, TeacherApplicationNominationError> {
    match idempotency_key {
        Some(value) => {
            let trimmed = value.trim().to_string();
            if trimmed.is_empty() {
                Err(TeacherApplicationNominationError::InvalidInput(
                    "idempotency_key cannot be blank".to_string(),
                ))
            } else {
                Ok(Some(trimmed))
            }
        }
        None => Ok(None),
    }
}
#[cfg(test)]
mod tests;
