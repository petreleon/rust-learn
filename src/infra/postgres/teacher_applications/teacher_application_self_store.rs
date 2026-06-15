use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::teacher_applications::get_my_application::{
    TeacherApplicationAuditEventOutput, TeacherApplicationOutput, TeacherApplicationSelfError,
    TeacherApplicationSelfStore,
};
use crate::infra::postgres::models::teacher_application::{
    TeacherApplication, TeacherApplicationAuditEvent,
};
use crate::infra::postgres::schema::{teacher_application_audit_events, teacher_applications};
use crate::infra::postgres::teacher_applications::teacher_application_audit_mappers::audit_event_output;
use crate::infra::postgres::teacher_applications::teacher_application_self_mappers::map_error;

pub struct PostgresTeacherApplicationSelfStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresTeacherApplicationSelfStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl TeacherApplicationSelfStore for PostgresTeacherApplicationSelfStore<'_> {
    fn find_latest_application_for_applicant(
        &mut self,
        applicant_user_id: i32,
    ) -> BoxFuture<'_, Result<Option<TeacherApplicationOutput>, TeacherApplicationSelfError>> {
        async move {
            teacher_applications::table
                .filter(teacher_applications::applicant_user_id.eq(applicant_user_id))
                .order(teacher_applications::created_at.desc())
                .then_order_by(teacher_applications::id.desc())
                .first::<TeacherApplication>(self.conn)
                .await
                .optional()
                .map(|application| application.map(Into::into))
                .map_err(map_error)
        }
        .boxed()
    }

    fn list_audit_events(
        &mut self,
        application_id: i64,
    ) -> BoxFuture<'_, Result<Vec<TeacherApplicationAuditEventOutput>, TeacherApplicationSelfError>>
    {
        async move {
            let events = teacher_application_audit_events::table
                .filter(teacher_application_audit_events::application_id.eq(application_id))
                .order(teacher_application_audit_events::created_at.asc())
                .load::<TeacherApplicationAuditEvent>(self.conn)
                .await
                .map_err(map_error)?;
            events
                .into_iter()
                .map(audit_event_output)
                .collect::<Result<Vec<_>, _>>()
                .map_err(TeacherApplicationSelfError::Database)
        }
        .boxed()
    }
}
