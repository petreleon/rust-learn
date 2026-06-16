use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::learning::manage_course_completion_terms::CourseCompletionTermsError;
use crate::domain::learning::course::{
    COURSE_TERMS_STATUS_ACTIVE, COURSE_TERMS_STATUS_COUNTERED, COURSE_TERMS_STATUS_SUBMITTED,
    COURSE_TERMS_STATUS_SUPERSEDED,
};
use crate::infra::postgres::models::course_completion_terms::CourseCompletionTerms;
use crate::infra::postgres::schema::course_completion_terms;

pub async fn supersede_open_terms(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    now: DateTime<Utc>,
) -> Result<Vec<CourseCompletionTerms>, CourseCompletionTermsError> {
    diesel::update(
        course_completion_terms::table
            .filter(course_completion_terms::course_id.eq(course_id))
            .filter(
                course_completion_terms::status
                    .eq_any([COURSE_TERMS_STATUS_SUBMITTED, COURSE_TERMS_STATUS_COUNTERED]),
            ),
    )
    .set((
        course_completion_terms::status.eq(COURSE_TERMS_STATUS_SUPERSEDED),
        course_completion_terms::superseded_at.eq(Some(now)),
        course_completion_terms::updated_at.eq(now),
    ))
    .get_results(conn)
    .await
    .map_err(map_supersede_error)
}

pub async fn supersede_active_terms(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    now: DateTime<Utc>,
) -> Result<Vec<CourseCompletionTerms>, CourseCompletionTermsError> {
    diesel::update(
        course_completion_terms::table
            .filter(course_completion_terms::course_id.eq(course_id))
            .filter(course_completion_terms::status.eq(COURSE_TERMS_STATUS_ACTIVE)),
    )
    .set((
        course_completion_terms::status.eq(COURSE_TERMS_STATUS_SUPERSEDED),
        course_completion_terms::superseded_at.eq(Some(now)),
        course_completion_terms::updated_at.eq(now),
    ))
    .get_results(conn)
    .await
    .map_err(map_supersede_error)
}

fn map_supersede_error(error: diesel::result::Error) -> CourseCompletionTermsError {
    match error {
        diesel::result::Error::NotFound => CourseCompletionTermsError::TermsNotFound,
        other => CourseCompletionTermsError::Database(other.to_string()),
    }
}
