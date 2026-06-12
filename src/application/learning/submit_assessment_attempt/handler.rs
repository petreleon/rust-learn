use chrono::Utc;

use crate::application::learning::assessment::CompletedAssessmentAttempt;
use crate::application::learning::ports::AssessmentSubmissionStore;
use crate::application::learning::submit_assessment_attempt::{
    AssessmentSubmissionError, SubmitAssessmentAttemptCommand, SubmitAssessmentAttemptOutput,
};
use crate::domain::learning::assessment::{score_answers, AssessmentQuestionAnswer};

pub async fn submit_assessment_attempt(
    store: &mut impl AssessmentSubmissionStore,
    command: SubmitAssessmentAttemptCommand,
) -> Result<SubmitAssessmentAttemptOutput, AssessmentSubmissionError> {
    let assessment = store
        .find_published_assessment(command.course_id, command.assessment_id)
        .await?;

    let previous_attempts = store
        .count_completed_attempts(command.assessment_id, command.user_id)
        .await
        .unwrap_or(0);

    if previous_attempts >= assessment.max_attempts as i64 {
        return Err(AssessmentSubmissionError::MaximumAttemptsReached);
    }

    let questions = store
        .list_questions_for_scoring(command.assessment_id)
        .await
        .unwrap_or_default();
    let questions = questions
        .into_iter()
        .map(|question| AssessmentQuestionAnswer {
            id: question.id,
            correct_answer: question.correct_answer,
            points: question.points,
        })
        .collect::<Vec<_>>();
    let score = score_answers(&questions, &command.answers);
    let passed = score.percentage >= assessment.passing_score;

    let attempt = store
        .create_completed_attempt(CompletedAssessmentAttempt {
            assessment_id: command.assessment_id,
            user_id: command.user_id,
            score: score.score,
            passed,
            completed_at: Utc::now(),
        })
        .await?;

    Ok(SubmitAssessmentAttemptOutput {
        attempt,
        score: score.score,
        total_points: score.total_points,
        percentage: score.percentage,
        passed,
    })
}
