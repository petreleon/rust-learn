use std::sync::Arc;

use actix_web::web;

use crate::application::teacher_applications::{
    notify_application_event::{
        TeacherApplicationNotificationCommand, TeacherApplicationNotificationUseCase,
    },
    TeacherApplicationOutput,
};

type TeacherApplicationNotificationData = web::Data<Arc<dyn TeacherApplicationNotificationUseCase>>;

pub(super) async fn notify_teacher_application_event(
    notifications: Option<&TeacherApplicationNotificationData>,
    application: &TeacherApplicationOutput,
    event_type: &str,
    reason: Option<&str>,
) {
    let Some(notifications) = notifications else {
        return;
    };
    let command = TeacherApplicationNotificationCommand {
        applicant_user_id: application.applicant_user_id,
        application_id: application.id,
        event_type: event_type.to_string(),
        organization_sponsor_id: application.organization_sponsor_id,
        reason: reason.map(ToOwned::to_owned),
        requested_organization_id: application.requested_organization_id,
        requested_scope: application.requested_scope.clone(),
        status: application.status.clone(),
    };

    if let Err(error) = notifications.notify_application_event(command).await {
        log::warn!(
            "event=teacher_application_notification_failed application_id={} error={:?}",
            application.id,
            error
        );
    }
}
