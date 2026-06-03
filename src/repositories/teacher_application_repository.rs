use crate::db::schema::{
    role_permission_organization, role_permission_platform, teacher_application_audit_events,
    teacher_applications, user_role_organization, user_role_platform,
};
use crate::models::teacher_application::{
    NewTeacherApplication, NewTeacherApplicationAuditEvent, TeacherApplication,
    TeacherApplicationAuditEvent,
};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

const DEFAULT_APPLICATION_LIMIT: i64 = 25;
const MAX_APPLICATION_LIMIT: i64 = 100;

#[derive(Debug, Clone, Default)]
pub struct TeacherApplicationFilter {
    pub status: Option<String>,
    pub applicant_user_id: Option<i32>,
    pub organization_sponsor_id: Option<i32>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl TeacherApplicationFilter {
    fn limit(&self) -> i64 {
        self.limit
            .unwrap_or(DEFAULT_APPLICATION_LIMIT)
            .clamp(1, MAX_APPLICATION_LIMIT)
    }

    fn offset(&self) -> i64 {
        self.offset.unwrap_or(0).max(0)
    }
}

pub async fn create_application(
    conn: &mut AsyncPgConnection,
    new_application: NewTeacherApplication,
) -> QueryResult<TeacherApplication> {
    diesel::insert_into(teacher_applications::table)
        .values(&new_application)
        .get_result(conn)
        .await
}

pub async fn find_application(
    conn: &mut AsyncPgConnection,
    application_id: i64,
) -> QueryResult<TeacherApplication> {
    teacher_applications::table
        .find(application_id)
        .first::<TeacherApplication>(conn)
        .await
}

pub async fn find_application_by_idempotency_key(
    conn: &mut AsyncPgConnection,
    idempotency_key: &str,
) -> QueryResult<Option<TeacherApplication>> {
    teacher_applications::table
        .filter(teacher_applications::idempotency_key.eq(idempotency_key))
        .first::<TeacherApplication>(conn)
        .await
        .optional()
}

pub async fn list_applications(
    conn: &mut AsyncPgConnection,
    filter: TeacherApplicationFilter,
) -> QueryResult<Vec<TeacherApplication>> {
    let mut query = teacher_applications::table.into_boxed();
    let limit = filter.limit();
    let offset = filter.offset();

    if let Some(status) = filter.status {
        query = query.filter(teacher_applications::status.eq(status));
    }

    if let Some(applicant_user_id) = filter.applicant_user_id {
        query = query.filter(teacher_applications::applicant_user_id.eq(applicant_user_id));
    }

    if let Some(organization_sponsor_id) = filter.organization_sponsor_id {
        query = query.filter(
            teacher_applications::organization_sponsor_id.eq(Some(organization_sponsor_id)),
        );
    }

    query
        .order(teacher_applications::created_at.desc())
        .limit(limit)
        .offset(offset)
        .load::<TeacherApplication>(conn)
        .await
}

pub async fn update_application_decision(
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

pub async fn create_audit_event(
    conn: &mut AsyncPgConnection,
    new_event: NewTeacherApplicationAuditEvent,
) -> QueryResult<TeacherApplicationAuditEvent> {
    diesel::insert_into(teacher_application_audit_events::table)
        .values(&new_event)
        .get_result(conn)
        .await
}

pub async fn list_audit_events(
    conn: &mut AsyncPgConnection,
    application_id: i64,
) -> QueryResult<Vec<TeacherApplicationAuditEvent>> {
    teacher_application_audit_events::table
        .filter(teacher_application_audit_events::application_id.eq(application_id))
        .order(teacher_application_audit_events::created_at.asc())
        .load::<TeacherApplicationAuditEvent>(conn)
        .await
}

pub async fn list_platform_user_ids_with_permission(
    conn: &mut AsyncPgConnection,
    permission: &str,
) -> QueryResult<Vec<i32>> {
    user_role_platform::table
        .inner_join(
            role_permission_platform::table.on(user_role_platform::platform_role_id
                .nullable()
                .eq(role_permission_platform::platform_role_id)),
        )
        .filter(role_permission_platform::permission.eq(permission))
        .select(user_role_platform::user_id.assume_not_null())
        .distinct()
        .load::<i32>(conn)
        .await
}

pub async fn list_organization_user_ids_with_permission(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
    permission: &str,
) -> QueryResult<Vec<i32>> {
    user_role_organization::table
        .inner_join(
            role_permission_organization::table.on(user_role_organization::organization_role_id
                .eq(role_permission_organization::organization_role_id)),
        )
        .filter(user_role_organization::organization_id.eq(Some(organization_id)))
        .filter(role_permission_organization::permission.eq(permission))
        .select(user_role_organization::user_id.assume_not_null())
        .distinct()
        .load::<i32>(conn)
        .await
}
