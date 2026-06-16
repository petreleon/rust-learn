use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::learning::manage_course_completion_terms::CourseCompletionTermsError;
use crate::domain::learning::course::{
    CourseCompletionTermsAuditEventType, CourseCompletionTermsStatus, COURSE_TERMS_STATUS_ACTIVE,
    COURSE_TERMS_STATUS_REJECTED, COURSE_TERMS_STATUS_WITHDRAWN,
};
use crate::infra::postgres::learning::course_completion_terms_mappers::new_audit_event;
use crate::infra::postgres::learning::course_completion_terms_records::insert_audit_event;
use crate::infra::postgres::models::course_completion_terms::CourseCompletionTerms;
use crate::infra::postgres::schema::course_completion_terms;

pub async fn mark_rejected(
    conn: &mut AsyncPgConnection,
    terms_id: i64,
    actor_user_id: i32,
    note: Option<String>,
    now: DateTime<Utc>,
) -> Result<CourseCompletionTerms, CourseCompletionTermsError> {
    transition_terms(
        conn,
        terms_id,
        actor_user_id,
        COURSE_TERMS_STATUS_REJECTED,
        CourseCompletionTermsAuditEventType::Rejected,
        note,
        now,
    )
    .await
}

pub async fn mark_withdrawn(
    conn: &mut AsyncPgConnection,
    terms_id: i64,
    actor_user_id: i32,
    note: Option<String>,
    now: DateTime<Utc>,
) -> Result<CourseCompletionTerms, CourseCompletionTermsError> {
    transition_terms(
        conn,
        terms_id,
        actor_user_id,
        COURSE_TERMS_STATUS_WITHDRAWN,
        CourseCompletionTermsAuditEventType::Withdrawn,
        note,
        now,
    )
    .await
}

pub async fn mark_active(
    conn: &mut AsyncPgConnection,
    terms_id: i64,
    actor_user_id: i32,
    reward_policy_id: i64,
    note: Option<String>,
    now: DateTime<Utc>,
) -> Result<CourseCompletionTerms, CourseCompletionTermsError> {
    let previous = find_terms(conn, terms_id).await?;
    let previous_status = CourseCompletionTermsStatus::parse(&previous.status)
        .map_err(|error| CourseCompletionTermsError::Database(error.to_string()))?;
    let updated = diesel::update(course_completion_terms::table.find(terms_id))
        .set((
            course_completion_terms::status.eq(COURSE_TERMS_STATUS_ACTIVE),
            course_completion_terms::reward_policy_id.eq(Some(reward_policy_id)),
            course_completion_terms::accepted_by_user_id.eq(Some(actor_user_id)),
            course_completion_terms::accepted_at.eq(Some(now)),
            course_completion_terms::activated_at.eq(Some(now)),
            course_completion_terms::updated_at.eq(now),
        ))
        .get_result::<CourseCompletionTerms>(conn)
        .await
        .map_err(map_transition_error)?;
    insert_audit_event(
        conn,
        new_audit_event(
            &updated,
            actor_user_id,
            CourseCompletionTermsAuditEventType::Accepted,
            Some(previous_status),
            note.clone(),
        ),
    )
    .await?;
    insert_audit_event(
        conn,
        new_audit_event(
            &updated,
            actor_user_id,
            CourseCompletionTermsAuditEventType::Activated,
            Some(CourseCompletionTermsStatus::Accepted),
            note,
        ),
    )
    .await?;
    Ok(updated)
}

async fn transition_terms(
    conn: &mut AsyncPgConnection,
    terms_id: i64,
    actor_user_id: i32,
    status: &str,
    event_type: CourseCompletionTermsAuditEventType,
    note: Option<String>,
    now: DateTime<Utc>,
) -> Result<CourseCompletionTerms, CourseCompletionTermsError> {
    let previous = find_terms(conn, terms_id).await?;
    let previous_status = CourseCompletionTermsStatus::parse(&previous.status)
        .map_err(|error| CourseCompletionTermsError::Database(error.to_string()))?;
    let updated = diesel::update(course_completion_terms::table.find(terms_id))
        .set((
            course_completion_terms::status.eq(status),
            course_completion_terms::updated_at.eq(now),
        ))
        .get_result::<CourseCompletionTerms>(conn)
        .await
        .map_err(map_transition_error)?;
    insert_audit_event(
        conn,
        new_audit_event(
            &updated,
            actor_user_id,
            event_type,
            Some(previous_status),
            note,
        ),
    )
    .await?;
    Ok(updated)
}

async fn find_terms(
    conn: &mut AsyncPgConnection,
    terms_id: i64,
) -> Result<CourseCompletionTerms, CourseCompletionTermsError> {
    course_completion_terms::table
        .find(terms_id)
        .first(conn)
        .await
        .map_err(map_transition_error)
}

fn map_transition_error(error: diesel::result::Error) -> CourseCompletionTermsError {
    match error {
        diesel::result::Error::NotFound => CourseCompletionTermsError::TermsNotFound,
        other => CourseCompletionTermsError::Database(other.to_string()),
    }
}
