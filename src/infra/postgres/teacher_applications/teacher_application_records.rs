use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::{teacher_application_audit_events, teacher_applications};
use crate::infra::postgres::models::teacher_application::{
    NewTeacherApplication, NewTeacherApplicationAuditEvent, TeacherApplication,
    TeacherApplicationAuditEvent,
};

pub(super) async fn create_application(
    conn: &mut AsyncPgConnection,
    new_application: NewTeacherApplication,
) -> QueryResult<TeacherApplication> {
    diesel::insert_into(teacher_applications::table)
        .values(&new_application)
        .get_result(conn)
        .await
}

pub(super) async fn find_application(
    conn: &mut AsyncPgConnection,
    application_id: i64,
) -> QueryResult<TeacherApplication> {
    teacher_applications::table
        .find(application_id)
        .first::<TeacherApplication>(conn)
        .await
}

pub(super) async fn find_application_by_idempotency_key(
    conn: &mut AsyncPgConnection,
    idempotency_key: &str,
) -> QueryResult<Option<TeacherApplication>> {
    teacher_applications::table
        .filter(teacher_applications::idempotency_key.eq(idempotency_key))
        .first::<TeacherApplication>(conn)
        .await
        .optional()
}

pub(super) async fn find_latest_application_for_applicant(
    conn: &mut AsyncPgConnection,
    applicant_id: i32,
) -> QueryResult<Option<TeacherApplication>> {
    teacher_applications::table
        .filter(teacher_applications::applicant_user_id.eq(applicant_id))
        .order(teacher_applications::created_at.desc())
        .then_order_by(teacher_applications::id.desc())
        .first::<TeacherApplication>(conn)
        .await
        .optional()
}

pub(super) async fn update_application_decision(
    conn: &mut AsyncPgConnection,
    application_id: i64,
    reviewer_id: i32,
    status: &str,
    decision_reason: Option<&str>,
    decided_at: DateTime<Utc>,
) -> QueryResult<TeacherApplication> {
    diesel::update(teacher_applications::table.find(application_id))
        .set((
            teacher_applications::status.eq(status),
            teacher_applications::reviewer_id.eq(Some(reviewer_id)),
            teacher_applications::decision_reason.eq(decision_reason),
            teacher_applications::updated_at.eq(decided_at),
            teacher_applications::decided_at.eq(Some(decided_at)),
        ))
        .get_result(conn)
        .await
}

pub(super) async fn create_audit_event(
    conn: &mut AsyncPgConnection,
    new_event: NewTeacherApplicationAuditEvent,
) -> QueryResult<TeacherApplicationAuditEvent> {
    diesel::insert_into(teacher_application_audit_events::table)
        .values(&new_event)
        .get_result(conn)
        .await
}
