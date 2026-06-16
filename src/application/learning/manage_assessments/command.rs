use crate::domain::learning::assessment::AssessmentQuestionOptions;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssessmentAuthoringCommand {
    pub actor_user_id: i32,
    pub course_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub passing_score: i32,
    pub max_attempts: i32,
    pub published: bool,
    pub questions: Vec<AssessmentQuestionCommand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssessmentUpdateCommand {
    pub actor_user_id: i32,
    pub assessment_id: i32,
    pub course_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub passing_score: i32,
    pub max_attempts: i32,
    pub published: bool,
    pub questions: Vec<AssessmentQuestionCommand>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssessmentQuestionCommand {
    pub text: String,
    pub question_type: String,
    pub options: Option<AssessmentQuestionOptions>,
    pub correct_answer: Option<String>,
    pub points: i32,
    pub order: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssessmentDraft {
    pub title: String,
    pub description: Option<String>,
    pub passing_score: i32,
    pub max_attempts: i32,
    pub published: bool,
    pub questions: Vec<AssessmentQuestionDraft>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssessmentQuestionDraft {
    pub text: String,
    pub question_type: String,
    pub options: Option<AssessmentQuestionOptions>,
    pub correct_answer: Option<String>,
    pub points: i32,
    pub order: i32,
}

impl From<AssessmentAuthoringCommand> for AssessmentDraft {
    fn from(command: AssessmentAuthoringCommand) -> Self {
        Self {
            title: command.title,
            description: command.description,
            passing_score: command.passing_score,
            max_attempts: command.max_attempts,
            published: command.published,
            questions: command.questions.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<AssessmentUpdateCommand> for AssessmentDraft {
    fn from(command: AssessmentUpdateCommand) -> Self {
        Self {
            title: command.title,
            description: command.description,
            passing_score: command.passing_score,
            max_attempts: command.max_attempts,
            published: command.published,
            questions: command.questions.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<AssessmentQuestionCommand> for AssessmentQuestionDraft {
    fn from(question: AssessmentQuestionCommand) -> Self {
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
