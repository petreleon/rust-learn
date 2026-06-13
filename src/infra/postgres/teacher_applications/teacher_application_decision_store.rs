use chrono::Utc;
use diesel_async::{AsyncConnection, AsyncPgConnection};
use futures::future::{BoxFuture, FutureExt};

use crate::application::teacher_applications::{
    decide_application::{TeacherApplicationDecisionError, TeacherApplicationDecisionStore},
    TeacherApplicationOutput,
};
use crate::domain::teacher_applications::status::TEACHER_APPLICATION_STATUS_APPROVED;
use crate::models::teacher_application::NewTeacherApplicationAuditEvent;
use crate::repositories::platform_repository::user_permission_platform_request;
use crate::repositories::teacher_application_repository;

use super::teacher_application_decision_roles::assign_approved_teaching_bundle;

pub struct PostgresTeacherApplicationDecisionStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresTeacherApplicationDecisionStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl TeacherApplicationDecisionStore for PostgresTeacherApplicationDecisionStore<'_> {
    fn has_platform_permission(
        &mut self,
        actor_user_id: i32,
        permission: String,
    ) -> BoxFuture<'_, Result<bool, TeacherApplicationDecisionError>> {
        async move {
            user_permission_platform_request(self.conn, actor_user_id, &permission)
                .await
                .map_err(map_error)
        }
        .boxed()
    }

    fn application(
        &mut self,
        application_id: i64,
    ) -> BoxFuture<'_, Result<TeacherApplicationOutput, TeacherApplicationDecisionError>> {
        async move {
            teacher_application_repository::find_application(self.conn, application_id)
                .await
                .map(Into::into)
                .map_err(map_error)
        }
        .boxed()
    }

    fn apply_decision(
        &mut self,
        actor_user_id: i32,
        current: TeacherApplicationOutput,
        target_status: String,
        decision_reason: Option<String>,
    ) -> BoxFuture<'_, Result<TeacherApplicationOutput, TeacherApplicationDecisionError>> {
        async move {
            self.conn
                .transaction::<_, diesel::result::Error, _>(|conn| {
                    Box::pin(async move {
                        let now = Utc::now();
                        let updated = teacher_application_repository::update_application_decision(
                            conn,
                            current.id,
                            actor_user_id,
                            &target_status,
                            decision_reason.as_deref(),
                            now,
                        )
                        .await?;
                        let updated_output: TeacherApplicationOutput = updated.clone().into();
                        if target_status == TEACHER_APPLICATION_STATUS_APPROVED {
                            assign_approved_teaching_bundle(conn, &updated_output).await?;
                        }
                        teacher_application_repository::create_audit_event(
                            conn,
                            NewTeacherApplicationAuditEvent {
                                application_id: current.id,
                                actor_user_id: Some(actor_user_id),
                                event_type: target_status.clone(),
                                from_status: Some(current.status),
                                to_status: target_status,
                                reason: decision_reason,
                            },
                        )
                        .await?;
                        Ok(updated_output)
                    })
                })
                .await
                .map_err(map_error)
        }
        .boxed()
    }
}

fn map_error(error: diesel::result::Error) -> TeacherApplicationDecisionError {
    match error {
        diesel::result::Error::NotFound => TeacherApplicationDecisionError::NotFound,
        other => TeacherApplicationDecisionError::Database(other.to_string()),
    }
}
