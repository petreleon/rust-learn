use actix_web::{web, HttpRequest, HttpResponse};
use std::collections::HashSet;

use crate::application::teacher_applications::TeacherApplicationOutput;
use crate::config::constants::permissions::Permissions;
use crate::db;
use crate::models::teacher_application::TeacherApplication;
use crate::repositories::teacher_application_repository::{
    list_organization_user_ids_with_permission, list_platform_user_ids_with_permission,
};
use crate::services::teacher_application_service::TeacherApplicationError;
use crate::utils::notifications::NotificationsState;

pub(super) fn service_error_response(error: TeacherApplicationError) -> HttpResponse {
    match error {
        TeacherApplicationError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have the required permission")
        }
        TeacherApplicationError::InvalidInput(message) => HttpResponse::BadRequest().body(message),
        TeacherApplicationError::InvalidTransition(message) => {
            HttpResponse::Conflict().body(message)
        }
        TeacherApplicationError::NotFound => {
            HttpResponse::NotFound().body("Teacher application not found")
        }
        TeacherApplicationError::Database(message) => {
            log::error!(
                "event=teacher_application_api_failed reason=database error={}",
                message
            );
            HttpResponse::InternalServerError().body("Failed to process teacher application")
        }
    }
}

pub(super) async fn notify_teacher_application_event(
    req: &HttpRequest,
    application: &TeacherApplicationNotification,
    event_type: &str,
    reason: Option<&str>,
) {
    let notifications = match req.app_data::<web::Data<NotificationsState>>() {
        Some(notifications) => notifications,
        None => return,
    };
    let pool = match req.app_data::<web::Data<db::DbPool>>() {
        Some(pool) => pool,
        None => return,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(error) => {
            log::warn!(
                "event=teacher_application_notification_connection_failed application_id={} error={}",
                application.id,
                error
            );
            return;
        }
    };

    let mut recipient_ids = HashSet::new();
    recipient_ids.insert(application.applicant_user_id);

    let platform_permission = Permissions::REVIEW_TEACHER_APPLICATIONS.to_string();
    match list_platform_user_ids_with_permission(&mut conn, &platform_permission).await {
        Ok(ids) => recipient_ids.extend(ids),
        Err(error) => log::warn!(
            "event=teacher_application_notification_recipient_lookup_failed scope=platform application_id={} error={}",
            application.id,
            error
        ),
    }

    if let Some(organization_id) = application
        .organization_sponsor_id
        .or(application.requested_organization_id)
    {
        let organization_permission = Permissions::VIEW_ORG_TEACHER_APPLICATIONS.to_string();
        match list_organization_user_ids_with_permission(
            &mut conn,
            organization_id,
            &organization_permission,
        )
        .await
        {
            Ok(ids) => recipient_ids.extend(ids),
            Err(error) => log::warn!(
                "event=teacher_application_notification_recipient_lookup_failed scope=organization application_id={} organization_id={} error={}",
                application.id,
                organization_id,
                error
            ),
        }
    }

    for recipient_id in recipient_ids {
        if let Err(error) = notifications
            .send_teacher_application_notification(
                recipient_id,
                application.id,
                event_type,
                application.status.as_str(),
                application.requested_scope.as_str(),
                reason,
            )
            .await
        {
            log::warn!(
                "event=teacher_application_notification_send_failed application_id={} recipient_id={} error={:?}",
                application.id,
                recipient_id,
                error
            );
        }
    }
}

pub(super) struct TeacherApplicationNotification {
    id: i64,
    applicant_user_id: i32,
    requested_organization_id: Option<i32>,
    organization_sponsor_id: Option<i32>,
    status: String,
    requested_scope: String,
}

impl From<&TeacherApplication> for TeacherApplicationNotification {
    fn from(application: &TeacherApplication) -> Self {
        Self {
            applicant_user_id: application.applicant_user_id,
            id: application.id,
            organization_sponsor_id: application.organization_sponsor_id,
            requested_organization_id: application.requested_organization_id,
            requested_scope: application.requested_scope.clone(),
            status: application.status.clone(),
        }
    }
}

impl From<&TeacherApplicationOutput> for TeacherApplicationNotification {
    fn from(application: &TeacherApplicationOutput) -> Self {
        Self {
            applicant_user_id: application.applicant_user_id,
            id: application.id,
            organization_sponsor_id: application.organization_sponsor_id,
            requested_organization_id: application.requested_organization_id,
            requested_scope: application.requested_scope.clone(),
            status: application.status.clone(),
        }
    }
}
