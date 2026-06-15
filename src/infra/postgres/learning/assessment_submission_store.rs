use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::assessment::{
    AssessmentAttemptOutput, AssessmentOutput, AssessmentQuestionForScoring,
    CompletedAssessmentAttempt,
};
use crate::application::learning::ports::AssessmentSubmissionStore;
use crate::application::learning::submit_assessment_attempt::AssessmentSubmissionError;
use crate::db::schema::{assessment_attempts, assessment_questions, assessments};
use crate::infra::postgres::models::assessment::{
    Assessment, AssessmentAttempt, AssessmentQuestion,
};

pub struct PostgresAssessmentSubmissionStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresAssessmentSubmissionStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl AssessmentSubmissionStore for PostgresAssessmentSubmissionStore<'_> {
    fn find_published_assessment(
        &mut self,
        course_id: i32,
        assessment_id: i32,
    ) -> BoxFuture<'_, Result<AssessmentOutput, AssessmentSubmissionError>> {
        async move {
            assessments::table
                .filter(assessments::id.eq(assessment_id))
                .filter(assessments::course_id.eq(course_id))
                .filter(assessments::published.eq(true))
                .first::<Assessment>(self.conn)
                .await
                .map(AssessmentOutput::from)
                .map_err(map_assessment_load_error)
        }
        .boxed()
    }

    fn count_completed_attempts(
        &mut self,
        assessment_id: i32,
        user_id: i32,
    ) -> BoxFuture<'_, Result<i64, AssessmentSubmissionError>> {
        async move {
            assessment_attempts::table
                .filter(assessment_attempts::assessment_id.eq(assessment_id))
                .filter(assessment_attempts::user_id.eq(user_id))
                .filter(assessment_attempts::completed_at.is_not_null())
                .count()
                .get_result(self.conn)
                .await
                .map_err(map_assessment_load_error)
        }
        .boxed()
    }

    fn list_questions_for_scoring(
        &mut self,
        assessment_id: i32,
    ) -> BoxFuture<'_, Result<Vec<AssessmentQuestionForScoring>, AssessmentSubmissionError>> {
        async move {
            assessment_questions::table
                .filter(assessment_questions::assessment_id.eq(assessment_id))
                .order(assessment_questions::order.asc())
                .load::<AssessmentQuestion>(self.conn)
                .await
                .map(|questions| {
                    questions
                        .into_iter()
                        .map(AssessmentQuestionForScoring::from)
                        .collect()
                })
                .map_err(map_assessment_load_error)
        }
        .boxed()
    }

    fn create_completed_attempt(
        &mut self,
        attempt: CompletedAssessmentAttempt,
    ) -> BoxFuture<'_, Result<AssessmentAttemptOutput, AssessmentSubmissionError>> {
        async move {
            diesel::insert_into(assessment_attempts::table)
                .values((
                    assessment_attempts::assessment_id.eq(attempt.assessment_id),
                    assessment_attempts::user_id.eq(attempt.user_id),
                    assessment_attempts::score.eq(attempt.score),
                    assessment_attempts::passed.eq(attempt.passed),
                    assessment_attempts::completed_at.eq(attempt.completed_at),
                ))
                .get_result::<AssessmentAttempt>(self.conn)
                .await
                .map(AssessmentAttemptOutput::from)
                .map_err(map_attempt_save_error)
        }
        .boxed()
    }
}

fn map_assessment_load_error(error: diesel::result::Error) -> AssessmentSubmissionError {
    match error {
        diesel::result::Error::NotFound => AssessmentSubmissionError::NotFound,
        other => AssessmentSubmissionError::LoadFailed(other.to_string()),
    }
}

fn map_attempt_save_error(error: diesel::result::Error) -> AssessmentSubmissionError {
    match error {
        diesel::result::Error::NotFound => AssessmentSubmissionError::NotFound,
        other => AssessmentSubmissionError::SaveFailed(other.to_string()),
    }
}
