use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::learning::manage_course_completion_terms::{
    CourseCompletionTermsCourseContext, CourseCompletionTermsError,
};
use crate::domain::learning::course::COURSE_TERMS_STATUS_ACTIVE;
use crate::infra::postgres::learning::course_completion_terms_mappers::map_terms_error;
use crate::infra::postgres::models::course_completion_term_audit_event::{
    CourseCompletionTermAuditEvent, NewCourseCompletionTermAuditEvent,
};
use crate::infra::postgres::models::course_completion_terms::{
    CourseCompletionTerms, NewCourseCompletionTerms,
};
use crate::infra::postgres::schema::{
    course_completion_term_audit_events, course_completion_terms, courses, courses_organizations,
};

pub async fn course_context(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<CourseCompletionTermsCourseContext, CourseCompletionTermsError> {
    courses::table
        .find(course_id)
        .select(courses::id)
        .first::<i32>(conn)
        .await
        .map_err(|error| match error {
            diesel::result::Error::NotFound => CourseCompletionTermsError::CourseNotFound,
            other => CourseCompletionTermsError::Database(other.to_string()),
        })?;
    Ok(CourseCompletionTermsCourseContext {
        course_id,
        organization_id: first_course_organization(conn, course_id).await?,
    })
}

pub async fn create_terms(
    conn: &mut AsyncPgConnection,
    terms: NewCourseCompletionTerms,
) -> Result<CourseCompletionTerms, CourseCompletionTermsError> {
    diesel::insert_into(course_completion_terms::table)
        .values(terms)
        .get_result(conn)
        .await
        .map_err(map_terms_error)
}

pub async fn insert_audit_event(
    conn: &mut AsyncPgConnection,
    event: NewCourseCompletionTermAuditEvent,
) -> Result<CourseCompletionTermAuditEvent, CourseCompletionTermsError> {
    diesel::insert_into(course_completion_term_audit_events::table)
        .values(event)
        .get_result(conn)
        .await
        .map_err(map_terms_error)
}

pub async fn next_version(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<i32, CourseCompletionTermsError> {
    let current = course_completion_terms::table
        .filter(course_completion_terms::course_id.eq(course_id))
        .select(diesel::dsl::max(course_completion_terms::version))
        .first::<Option<i32>>(conn)
        .await
        .map_err(map_terms_error)?;
    Ok(current.unwrap_or(0) + 1)
}

pub async fn terms_by_id(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    terms_id: i64,
) -> Result<Option<CourseCompletionTerms>, CourseCompletionTermsError> {
    course_completion_terms::table
        .filter(course_completion_terms::id.eq(terms_id))
        .filter(course_completion_terms::course_id.eq(course_id))
        .first(conn)
        .await
        .optional()
        .map_err(map_terms_error)
}

pub async fn terms_by_terms_id(
    conn: &mut AsyncPgConnection,
    terms_id: i64,
) -> Result<CourseCompletionTerms, CourseCompletionTermsError> {
    course_completion_terms::table
        .find(terms_id)
        .first(conn)
        .await
        .map_err(map_terms_error)
}

pub async fn active_terms(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<Option<CourseCompletionTerms>, CourseCompletionTermsError> {
    course_completion_terms::table
        .filter(course_completion_terms::course_id.eq(course_id))
        .filter(course_completion_terms::status.eq(COURSE_TERMS_STATUS_ACTIVE))
        .first(conn)
        .await
        .optional()
        .map_err(map_terms_error)
}

pub async fn list_terms(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<Vec<CourseCompletionTerms>, CourseCompletionTermsError> {
    course_completion_terms::table
        .filter(course_completion_terms::course_id.eq(course_id))
        .order(course_completion_terms::version.desc())
        .load(conn)
        .await
        .map_err(map_terms_error)
}

pub async fn list_audit_events(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<Vec<CourseCompletionTermAuditEvent>, CourseCompletionTermsError> {
    course_completion_term_audit_events::table
        .filter(course_completion_term_audit_events::course_id.eq(course_id))
        .order(course_completion_term_audit_events::created_at.desc())
        .load(conn)
        .await
        .map_err(map_terms_error)
}

async fn first_course_organization(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<Option<i32>, CourseCompletionTermsError> {
    courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .select(courses_organizations::organization_id)
        .first(conn)
        .await
        .optional()
        .map_err(map_terms_error)
}
