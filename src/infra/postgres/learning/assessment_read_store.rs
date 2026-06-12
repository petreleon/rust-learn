use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::assessment::{
    AssessmentAttemptOutput, AssessmentOutput, AssessmentReadError,
};
use crate::application::learning::ports::AssessmentReadStore;
use crate::db::schema::{assessment_attempts, assessments};
use crate::models::assessment::{Assessment, AssessmentAttempt};

pub struct PostgresAssessmentReadStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresAssessmentReadStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl AssessmentReadStore for PostgresAssessmentReadStore<'_> {
    fn list_published_for_course(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<AssessmentOutput>, AssessmentReadError>> {
        async move {
            assessments::table
                .filter(assessments::course_id.eq(course_id))
                .filter(assessments::published.eq(true))
                .load::<Assessment>(self.conn)
                .await
                .map(|assessments| {
                    assessments
                        .into_iter()
                        .map(AssessmentOutput::from)
                        .collect()
                })
                .map_err(map_assessment_read_error)
        }
        .boxed()
    }

    fn list_attempts_for_user(
        &mut self,
        assessment_id: i32,
        user_id: i32,
    ) -> BoxFuture<'_, Result<Vec<AssessmentAttemptOutput>, AssessmentReadError>> {
        async move {
            assessment_attempts::table
                .filter(assessment_attempts::assessment_id.eq(assessment_id))
                .filter(assessment_attempts::user_id.eq(user_id))
                .order(assessment_attempts::started_at.desc())
                .load::<AssessmentAttempt>(self.conn)
                .await
                .map(|attempts| {
                    attempts
                        .into_iter()
                        .map(AssessmentAttemptOutput::from)
                        .collect()
                })
                .map_err(map_assessment_read_error)
        }
        .boxed()
    }
}

fn map_assessment_read_error(error: diesel::result::Error) -> AssessmentReadError {
    AssessmentReadError::Database(error.to_string())
}
