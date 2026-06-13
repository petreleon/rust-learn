use crate::application::learning::assessment::{
    AssessmentAttemptOutput, AssessmentOutput, AssessmentQuestionForScoring,
};
use crate::application::learning::create_course::CourseCreationOutput;
use crate::models::assessment::{Assessment, AssessmentAttempt, AssessmentQuestion};
use crate::models::course::Course;

impl From<Course> for CourseCreationOutput {
    fn from(course: Course) -> Self {
        Self {
            id: course.id,
            title: course.title,
            lifecycle_status: course.lifecycle_status,
            description: course.description,
            topics: course.topics,
            prerequisites: course.prerequisites,
        }
    }
}

impl From<Assessment> for AssessmentOutput {
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
        }
    }
}

impl From<AssessmentAttempt> for AssessmentAttemptOutput {
    fn from(attempt: AssessmentAttempt) -> Self {
        Self {
            id: attempt.id,
            assessment_id: attempt.assessment_id,
            user_id: attempt.user_id,
            score: attempt.score,
            passed: attempt.passed,
            started_at: attempt.started_at,
            completed_at: attempt.completed_at,
        }
    }
}

impl From<AssessmentQuestion> for AssessmentQuestionForScoring {
    fn from(question: AssessmentQuestion) -> Self {
        Self {
            id: question.id,
            correct_answer: question.correct_answer,
            points: question.points,
        }
    }
}
