use std::collections::BTreeSet;

use super::{
    TeacherApplicationNotificationCommand, TeacherApplicationNotificationError,
    TeacherApplicationNotificationOutcome, TeacherApplicationNotificationStore,
};

pub async fn notify_application_event(
    store: &mut impl TeacherApplicationNotificationStore,
    command: TeacherApplicationNotificationCommand,
) -> Result<TeacherApplicationNotificationOutcome, TeacherApplicationNotificationError> {
    let mut failed_count = 0;
    let mut recipients = BTreeSet::from([command.applicant_user_id]);

    match store.platform_reviewer_ids().await {
        Ok(ids) => recipients.extend(ids),
        Err(error) => {
            failed_count += 1;
            log_recipient_lookup_error("platform", command.application_id, None, &error);
        }
    }

    if let Some(organization_id) = command
        .organization_sponsor_id
        .or(command.requested_organization_id)
    {
        match store.organization_viewer_ids(organization_id).await {
            Ok(ids) => recipients.extend(ids),
            Err(error) => {
                failed_count += 1;
                log_recipient_lookup_error(
                    "organization",
                    command.application_id,
                    Some(organization_id),
                    &error,
                );
            }
        }
    }

    let recipient_count = recipients.len();
    let mut sent_count = 0;
    for recipient_id in recipients {
        match store.send(recipient_id, command.clone()).await {
            Ok(()) => sent_count += 1,
            Err(error) => {
                failed_count += 1;
                log::warn!(
                    "event=teacher_application_notification_send_failed application_id={} recipient_id={} error={:?}",
                    command.application_id,
                    recipient_id,
                    error
                );
            }
        }
    }

    Ok(TeacherApplicationNotificationOutcome {
        recipient_count,
        sent_count,
        failed_count,
    })
}

fn log_recipient_lookup_error(
    scope: &str,
    application_id: i64,
    organization_id: Option<i32>,
    error: &TeacherApplicationNotificationError,
) {
    log::warn!(
        "event=teacher_application_notification_recipient_lookup_failed scope={} application_id={} organization_id={:?} error={:?}",
        scope,
        application_id,
        organization_id,
        error
    );
}

#[cfg(test)]
mod tests;
