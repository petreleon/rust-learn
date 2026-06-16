use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::learning::manage_assessments::{AssessmentDraft, AssessmentQuestionDraft};
use crate::infra::postgres::models::assessment::Assessment;
use crate::infra::postgres::schema::{assessment_questions, assessments};

pub(in crate::infra::postgres::learning) async fn insert_assessment(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    draft: AssessmentDraft,
) -> diesel::QueryResult<Assessment> {
    let questions = draft.questions;
    let assessment = diesel::insert_into(assessments::table)
        .values((
            assessments::course_id.eq(course_id),
            assessments::title.eq(draft.title),
            assessments::description.eq(draft.description),
            assessments::passing_score.eq(draft.passing_score),
            assessments::max_attempts.eq(draft.max_attempts),
            assessments::published.eq(draft.published),
        ))
        .get_result::<Assessment>(conn)
        .await?;
    insert_questions(conn, assessment.id, questions).await?;
    Ok(assessment)
}

pub(in crate::infra::postgres::learning) async fn update_assessment_row(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    assessment_id: i32,
    draft: &AssessmentDraft,
) -> diesel::QueryResult<Assessment> {
    diesel::update(
        assessments::table
            .filter(assessments::id.eq(assessment_id))
            .filter(assessments::course_id.eq(course_id)),
    )
    .set((
        assessments::title.eq(draft.title.clone()),
        assessments::description.eq(draft.description.clone()),
        assessments::passing_score.eq(draft.passing_score),
        assessments::max_attempts.eq(draft.max_attempts),
        assessments::published.eq(draft.published),
        assessments::updated_at.eq(diesel::dsl::now),
    ))
    .get_result::<Assessment>(conn)
    .await
}

pub(in crate::infra::postgres::learning) async fn replace_questions(
    conn: &mut AsyncPgConnection,
    assessment_id: i32,
    questions: Vec<AssessmentQuestionDraft>,
) -> diesel::QueryResult<()> {
    diesel::delete(
        assessment_questions::table.filter(assessment_questions::assessment_id.eq(assessment_id)),
    )
    .execute(conn)
    .await?;
    insert_questions(conn, assessment_id, questions).await
}

async fn insert_questions(
    conn: &mut AsyncPgConnection,
    assessment_id: i32,
    questions: Vec<AssessmentQuestionDraft>,
) -> diesel::QueryResult<()> {
    for question in questions {
        diesel::insert_into(assessment_questions::table)
            .values(NewAssessmentQuestion::from((assessment_id, question)))
            .execute(conn)
            .await?;
    }
    Ok(())
}

#[derive(Insertable)]
#[diesel(table_name = assessment_questions)]
struct NewAssessmentQuestion {
    assessment_id: i32,
    text: String,
    question_type: String,
    options: Option<serde_json::Value>,
    correct_answer: Option<String>,
    points: i32,
    order: i32,
}

impl From<(i32, AssessmentQuestionDraft)> for NewAssessmentQuestion {
    fn from((assessment_id, question): (i32, AssessmentQuestionDraft)) -> Self {
        Self {
            assessment_id,
            text: question.text,
            question_type: question.question_type,
            options: question.options,
            correct_answer: question.correct_answer,
            points: question.points,
            order: question.order,
        }
    }
}
