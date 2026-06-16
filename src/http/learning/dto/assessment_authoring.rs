use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::learning::manage_assessments::{
    AssessmentAuthoringCommand, AssessmentQuestionCommand, AssessmentUpdateCommand,
    AuthoredAssessmentOutput, AuthoredAssessmentQuestionOutput,
};
use crate::domain::learning::assessment::AssessmentQuestionOptions;

#[derive(Debug, Clone, Serialize)]
pub struct AssessmentAuthoringResponse {
    pub id: i32,
    pub course_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub passing_score: i32,
    pub max_attempts: i32,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub questions: Vec<AssessmentAuthoringQuestionResponse>,
}

impl From<AuthoredAssessmentOutput> for AssessmentAuthoringResponse {
    fn from(assessment: AuthoredAssessmentOutput) -> Self {
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
            questions: assessment
                .questions
                .into_iter()
                .map(AssessmentAuthoringQuestionResponse::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AssessmentAuthoringQuestionResponse {
    pub id: i32,
    pub assessment_id: i32,
    pub text: String,
    pub question_type: String,
    pub options: Option<AssessmentQuestionOptions>,
    pub correct_answer: Option<String>,
    pub points: i32,
    pub order: i32,
}

impl From<AuthoredAssessmentQuestionOutput> for AssessmentAuthoringQuestionResponse {
    fn from(question: AuthoredAssessmentQuestionOutput) -> Self {
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

#[derive(Debug, Clone, Deserialize)]
pub struct AssessmentAuthoringRequest {
    pub title: String,
    pub description: Option<String>,
    pub passing_score: i32,
    pub max_attempts: i32,
    pub published: bool,
    pub questions: Vec<AssessmentQuestionAuthoringRequest>,
}

impl AssessmentAuthoringRequest {
    pub fn into_create_command(
        self,
        actor_user_id: i32,
        course_id: i32,
    ) -> AssessmentAuthoringCommand {
        AssessmentAuthoringCommand {
            actor_user_id,
            course_id,
            title: self.title,
            description: self.description,
            passing_score: self.passing_score,
            max_attempts: self.max_attempts,
            published: self.published,
            questions: self.questions.into_iter().map(Into::into).collect(),
        }
    }

    pub fn into_update_command(
        self,
        actor_user_id: i32,
        course_id: i32,
        assessment_id: i32,
    ) -> AssessmentUpdateCommand {
        AssessmentUpdateCommand {
            actor_user_id,
            assessment_id,
            course_id,
            title: self.title,
            description: self.description,
            passing_score: self.passing_score,
            max_attempts: self.max_attempts,
            published: self.published,
            questions: self.questions.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AssessmentQuestionAuthoringRequest {
    pub text: String,
    pub question_type: String,
    pub options: Option<AssessmentQuestionOptions>,
    pub correct_answer: Option<String>,
    pub points: i32,
    pub order: i32,
}

impl From<AssessmentQuestionAuthoringRequest> for AssessmentQuestionCommand {
    fn from(question: AssessmentQuestionAuthoringRequest) -> Self {
        Self {
            text: question.text,
            question_type: question.question_type,
            options: question.options,
            correct_answer: question.correct_answer,
            points: question.points,
            order: question.order,
        }
    }
}
