use diesel_async::{AsyncConnection, AsyncPgConnection};
use futures::future::{BoxFuture, FutureExt};

use crate::application::teacher_applications::{
    nominate_application::{TeacherApplicationNominationError, TeacherApplicationNominationStore},
    submit_application::TeacherApplicationSubmission,
    TeacherApplicationOutput,
};
use crate::models::teacher_application::{NewTeacherApplication, NewTeacherApplicationAuditEvent};
use crate::repositories::organization_repository::user_permission_organization_request;
use crate::repositories::teacher_application_repository;

pub struct PostgresTeacherApplicationNominationStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresTeacherApplicationNominationStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl TeacherApplicationNominationStore for PostgresTeacherApplicationNominationStore<'_> {
    fn has_organization_permission(
        &mut self,
        actor_user_id: i32,
        organization_id: i32,
        permission: String,
    ) -> BoxFuture<'_, Result<bool, TeacherApplicationNominationError>> {
        async move {
            user_permission_organization_request(
                self.conn,
                actor_user_id,
                organization_id,
                &permission,
            )
            .await
            .map_err(map_error)
        }
        .boxed()
    }

    fn find_application_by_idempotency_key(
        &mut self,
        idempotency_key: String,
    ) -> BoxFuture<'_, Result<Option<TeacherApplicationOutput>, TeacherApplicationNominationError>>
    {
        async move {
            teacher_application_repository::find_application_by_idempotency_key(
                self.conn,
                &idempotency_key,
            )
            .await
            .map(|application| application.map(Into::into))
            .map_err(map_error)
        }
        .boxed()
    }

    fn find_latest_application_for_applicant(
        &mut self,
        applicant_user_id: i32,
    ) -> BoxFuture<'_, Result<Option<TeacherApplicationOutput>, TeacherApplicationNominationError>>
    {
        async move {
            teacher_application_repository::find_latest_application_for_applicant(
                self.conn,
                applicant_user_id,
            )
            .await
            .map(|application| application.map(Into::into))
            .map_err(map_error)
        }
        .boxed()
    }

    fn create_organization_nomination(
        &mut self,
        actor_user_id: i32,
        submission: TeacherApplicationSubmission,
    ) -> BoxFuture<'_, Result<TeacherApplicationOutput, TeacherApplicationNominationError>> {
        async move {
            self.conn
                .transaction::<_, diesel::result::Error, _>(|conn| {
                    Box::pin(async move {
                        let application = teacher_application_repository::create_application(
                            conn,
                            new_application(submission),
                        )
                        .await?;
                        teacher_application_repository::create_audit_event(
                            conn,
                            NewTeacherApplicationAuditEvent {
                                application_id: application.id,
                                actor_user_id: Some(actor_user_id),
                                event_type: "organization_nominated".to_string(),
                                from_status: None,
                                to_status: application.status.clone(),
                                reason: None,
                            },
                        )
                        .await?;
                        Ok(application)
                    })
                })
                .await
                .map(Into::into)
                .map_err(map_error)
        }
        .boxed()
    }
}

fn new_application(submission: TeacherApplicationSubmission) -> NewTeacherApplication {
    NewTeacherApplication {
        applicant_user_id: submission.applicant_user_id,
        requested_scope: submission.requested_scope,
        requested_organization_id: submission.requested_organization_id,
        requested_course_id: submission.requested_course_id,
        experience_summary: submission.experience_summary,
        organization_sponsor_id: submission.organization_sponsor_id,
        portfolio_links: submission.portfolio_links,
        status: submission.status,
        idempotency_key: submission.idempotency_key,
    }
}

fn map_error(error: diesel::result::Error) -> TeacherApplicationNominationError {
    match error {
        diesel::result::Error::NotFound => TeacherApplicationNominationError::NotFound,
        other => TeacherApplicationNominationError::Database(other.to_string()),
    }
}
