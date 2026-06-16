use crate::application::learning::manage_assessments::{
    AssessmentAuthoringError, AssessmentDraft, AssessmentQuestionDraft,
};

pub(super) fn validate_draft(draft: &AssessmentDraft) -> Result<(), AssessmentAuthoringError> {
    if draft.title.trim().is_empty() {
        return Err(validation_error("assessment title is required"));
    }
    if !(0..=100).contains(&draft.passing_score) {
        return Err(validation_error("passing score must be between 0 and 100"));
    }
    if draft.max_attempts < 1 {
        return Err(validation_error("max attempts must be at least 1"));
    }
    if draft.published && draft.questions.is_empty() {
        return Err(validation_error("published assessments require questions"));
    }
    for question in &draft.questions {
        validate_question(question, draft.published)?;
    }
    Ok(())
}

fn validate_question(
    question: &AssessmentQuestionDraft,
    published: bool,
) -> Result<(), AssessmentAuthoringError> {
    if question.text.trim().is_empty() {
        return Err(validation_error("question text is required"));
    }
    if question.question_type.trim().is_empty() {
        return Err(validation_error("question type is required"));
    }
    if question.points < 1 {
        return Err(validation_error("question points must be at least 1"));
    }
    if question.order < 0 {
        return Err(validation_error("question order cannot be negative"));
    }
    if published
        && question
            .correct_answer
            .as_deref()
            .unwrap_or("")
            .trim()
            .is_empty()
    {
        return Err(validation_error(
            "published questions require correct answers",
        ));
    }
    Ok(())
}

fn validation_error(message: &str) -> AssessmentAuthoringError {
    AssessmentAuthoringError::Validation(message.to_string())
}
