use futures::future::BoxFuture;

use crate::application::learning::assessment::{AssessmentOutput, AssessmentReadError};

pub trait CourseAssessmentsUseCase: Send + Sync {
    fn list_published_course_assessments(
        &self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Vec<AssessmentOutput>, AssessmentReadError>>;
}
