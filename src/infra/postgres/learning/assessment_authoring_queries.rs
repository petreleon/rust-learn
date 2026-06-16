use std::collections::BTreeMap;

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::learning::manage_assessments::{
    AssessmentAuthoringError, AuthoredAssessmentOutput, AuthoredAssessmentQuestionOutput,
};
use crate::infra::postgres::models::assessment::{Assessment, AssessmentQuestion};
use crate::infra::postgres::schema::assessment_questions;

pub(in crate::infra::postgres::learning) async fn load_assessment_with_questions(
    conn: &mut AsyncPgConnection,
    assessment: Assessment,
) -> diesel::QueryResult<AuthoredAssessmentOutput> {
    let questions = assessment_questions::table
        .filter(assessment_questions::assessment_id.eq(assessment.id))
        .order(assessment_questions::order.asc())
        .load::<AssessmentQuestion>(conn)
        .await?;
    let mut output = AuthoredAssessmentOutput::from(assessment);
    output.questions = questions
        .into_iter()
        .map(AuthoredAssessmentQuestionOutput::from)
        .collect();
    Ok(output)
}

pub(in crate::infra::postgres::learning) async fn load_assessments_with_questions(
    conn: &mut AsyncPgConnection,
    assessments: Vec<Assessment>,
) -> Result<Vec<AuthoredAssessmentOutput>, AssessmentAuthoringError> {
    let assessment_ids = assessments
        .iter()
        .map(|assessment| assessment.id)
        .collect::<Vec<_>>();
    let mut questions = questions_by_assessment(conn, assessment_ids).await?;

    Ok(assessments
        .into_iter()
        .map(|assessment| {
            let assessment_id = assessment.id;
            let mut output = AuthoredAssessmentOutput::from(assessment);
            output.questions = questions.remove(&assessment_id).unwrap_or_default();
            output
        })
        .collect())
}

async fn questions_by_assessment(
    conn: &mut AsyncPgConnection,
    assessment_ids: Vec<i32>,
) -> Result<BTreeMap<i32, Vec<AuthoredAssessmentQuestionOutput>>, AssessmentAuthoringError> {
    if assessment_ids.is_empty() {
        return Ok(BTreeMap::new());
    }
    let rows = assessment_questions::table
        .filter(assessment_questions::assessment_id.eq_any(assessment_ids))
        .order((
            assessment_questions::assessment_id.asc(),
            assessment_questions::order.asc(),
        ))
        .load::<AssessmentQuestion>(conn)
        .await
        .map_err(map_authoring_error)?;

    let mut grouped = BTreeMap::<i32, Vec<AuthoredAssessmentQuestionOutput>>::new();
    for question in rows {
        grouped
            .entry(question.assessment_id)
            .or_default()
            .push(AuthoredAssessmentQuestionOutput::from(question));
    }
    Ok(grouped)
}

pub(in crate::infra::postgres::learning) fn map_authoring_error(
    error: diesel::result::Error,
) -> AssessmentAuthoringError {
    match error {
        diesel::result::Error::NotFound => AssessmentAuthoringError::NotFound,
        other => AssessmentAuthoringError::Database(other.to_string()),
    }
}

impl From<Assessment> for AuthoredAssessmentOutput {
    fn from(assessment: Assessment) -> Self {
        Self {
            id: assessment.id,
            course_id: assessment.course_id,
            title: assessment.title,
            description: assessment.description,
            passing_score: assessment.passing_score,
            max_attempts: assessment.max_attempts,
            published: assessment.published,
            created_at: assessment.created_at,
            updated_at: assessment.updated_at,
            questions: Vec::new(),
        }
    }
}

impl From<AssessmentQuestion> for AuthoredAssessmentQuestionOutput {
    fn from(question: AssessmentQuestion) -> Self {
        Self {
            id: question.id,
            assessment_id: question.assessment_id,
            text: question.text,
            question_type: question.question_type,
            options: question.options,
            correct_answer: question.correct_answer,
            points: question.points,
            order: question.order,
        }
    }
}
