use crate::application::teacher_applications::{
    decide_application::{
        TeacherApplicationDecisionCommand, TeacherApplicationDecisionError,
        TeacherApplicationDecisionStore,
    },
    TeacherApplicationOutput,
};
use crate::domain::teacher_applications::status::{
    normalize_decision_status, TEACHER_APPLICATION_STATUS_APPROVED,
    TEACHER_APPLICATION_STATUS_NEEDS_CHANGES, TEACHER_APPLICATION_STATUS_REJECTED,
};

pub async fn decide_application(
    store: &mut impl TeacherApplicationDecisionStore,
    command: TeacherApplicationDecisionCommand,
) -> Result<TeacherApplicationOutput, TeacherApplicationDecisionError> {
    let target_status = normalize_decision_status(&command.status)?;
    let permission = required_permission(&target_status)?;
    if !store
        .has_platform_permission(command.actor_user_id, permission.to_string())
        .await?
    {
        return Err(TeacherApplicationDecisionError::PermissionDenied(
            permission.to_string(),
        ));
    }

    let current = store.application(command.application_id).await?;
    if is_final_status(&current.status) {
        return Err(TeacherApplicationDecisionError::InvalidTransition(
            "final teacher applications cannot be changed".to_string(),
        ));
    }
    let from_status = current.status.clone();

    let application = store
        .apply_decision(
            command.actor_user_id,
            current,
            target_status,
            command.decision_reason,
        )
        .await?;
    log::info!(
        "event=teacher_application_transition application_id={} actor_user_id={} applicant_user_id={} transition={}->{} from_status={} to_status={} requested_scope={} organization_sponsor_id={:?} requested_organization_id={:?} requested_course_id={:?}",
        application.id,
        command.actor_user_id,
        application.applicant_user_id,
        from_status,
        application.status,
        from_status,
        application.status,
        application.requested_scope,
        application.organization_sponsor_id,
        application.requested_organization_id,
        application.requested_course_id
    );

    Ok(application)
}

fn required_permission(
    target_status: &str,
) -> Result<&'static str, TeacherApplicationDecisionError> {
    match target_status {
        TEACHER_APPLICATION_STATUS_APPROVED => Ok("APPROVE_TEACHER_APPLICATION"),
        TEACHER_APPLICATION_STATUS_REJECTED => Ok("REJECT_TEACHER_APPLICATION"),
        TEACHER_APPLICATION_STATUS_NEEDS_CHANGES => Ok("REVIEW_TEACHER_APPLICATIONS"),
        _ => Err(TeacherApplicationDecisionError::InvalidInput(
            "unsupported teacher application decision".to_string(),
        )),
    }
}

fn is_final_status(status: &str) -> bool {
    status == TEACHER_APPLICATION_STATUS_APPROVED || status == TEACHER_APPLICATION_STATUS_REJECTED
}

#[cfg(test)]
mod tests;
