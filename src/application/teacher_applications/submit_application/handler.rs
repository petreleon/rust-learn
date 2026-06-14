use crate::application::teacher_applications::{
    submit_application::{
        TeacherApplicationSubmission, TeacherApplicationSubmitCommand,
        TeacherApplicationSubmitError, TeacherApplicationSubmitStore,
    },
    TeacherApplicationOutput,
};
use crate::domain::teacher_applications::{
    scope::{normalize_scope, validate_requested_scope},
    status::{TEACHER_APPLICATION_STATUS_REJECTED, TEACHER_APPLICATION_STATUS_SUBMITTED},
};

pub async fn submit_application(
    store: &mut impl TeacherApplicationSubmitStore,
    command: TeacherApplicationSubmitCommand,
) -> Result<TeacherApplicationOutput, TeacherApplicationSubmitError> {
    if !store
        .can_submit_teacher_application(command.actor_user_id)
        .await?
    {
        return Err(TeacherApplicationSubmitError::PermissionDenied(
            "SUBMIT_TEACHER_APPLICATION".to_string(),
        ));
    }

    let submission = build_submission(command)?;
    if let Some(existing) = find_idempotent_application(store, &submission).await? {
        log::info!(
            "event=teacher_application_idempotent_replay application_id={} actor_user_id={} applicant_user_id={} status={} requested_scope={} idempotency_key={}",
            existing.id,
            submission.applicant_user_id,
            existing.applicant_user_id,
            existing.status,
            existing.requested_scope,
            existing.idempotency_key.as_deref().unwrap_or("none")
        );
        return Ok(existing);
    }
    ensure_no_blocking_application(store, &submission).await?;

    let application = store
        .create_submitted_application(submission.applicant_user_id, submission)
        .await?;
    log::info!(
        "event=teacher_application_transition application_id={} actor_user_id={} applicant_user_id={} transition=submitted from_status=none to_status={} requested_scope={} organization_sponsor_id={:?} requested_organization_id={:?} requested_course_id={:?}",
        application.id,
        application.applicant_user_id,
        application.applicant_user_id,
        application.status,
        application.requested_scope,
        application.organization_sponsor_id,
        application.requested_organization_id,
        application.requested_course_id
    );

    Ok(application)
}

fn build_submission(
    command: TeacherApplicationSubmitCommand,
) -> Result<TeacherApplicationSubmission, TeacherApplicationSubmitError> {
    let requested_scope = normalize_scope(&command.requested_scope)?;
    let experience_summary = command.experience_summary.trim().to_string();
    if experience_summary.is_empty() {
        return Err(TeacherApplicationSubmitError::InvalidInput(
            "experience_summary is required".to_string(),
        ));
    }
    validate_requested_scope(
        &requested_scope,
        command.requested_organization_id,
        command.requested_course_id,
        command.organization_sponsor_id,
    )?;

    Ok(TeacherApplicationSubmission {
        applicant_user_id: command.actor_user_id,
        requested_scope,
        requested_organization_id: command.requested_organization_id,
        requested_course_id: command.requested_course_id,
        experience_summary,
        organization_sponsor_id: command.organization_sponsor_id,
        portfolio_links: serde_json::json!(clean_portfolio_links(command.portfolio_links)),
        idempotency_key: normalize_idempotency_key(command.idempotency_key)?,
        status: TEACHER_APPLICATION_STATUS_SUBMITTED.to_string(),
    })
}

async fn find_idempotent_application(
    store: &mut impl TeacherApplicationSubmitStore,
    submission: &TeacherApplicationSubmission,
) -> Result<Option<TeacherApplicationOutput>, TeacherApplicationSubmitError> {
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
    store: &mut impl TeacherApplicationSubmitStore,
    submission: &TeacherApplicationSubmission,
) -> Result<(), TeacherApplicationSubmitError> {
    let existing = store
        .find_latest_application_for_applicant(submission.applicant_user_id)
        .await?;
    match existing {
        Some(application) if application.status != TEACHER_APPLICATION_STATUS_REJECTED => {
            Err(TeacherApplicationSubmitError::InvalidTransition(format!(
                "teacher application already exists with status {}",
                application.status
            )))
        }
        _ => Ok(()),
    }
}

fn ensure_idempotent_application_matches(
    existing: &TeacherApplicationOutput,
    submission: &TeacherApplicationSubmission,
) -> Result<(), TeacherApplicationSubmitError> {
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
        Err(TeacherApplicationSubmitError::InvalidInput(
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
) -> Result<Option<String>, TeacherApplicationSubmitError> {
    match idempotency_key {
        Some(value) => {
            let trimmed = value.trim().to_string();
            if trimmed.is_empty() {
                Err(TeacherApplicationSubmitError::InvalidInput(
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
