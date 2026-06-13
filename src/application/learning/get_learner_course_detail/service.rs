use futures::future::BoxFuture;

use crate::application::learning::get_learner_course_detail::{
    LearnerCourseDetailOutput, LearnerCourseDetailQuery,
};
use crate::application::learning::learner_course_catalog::LearnerCourseCatalogError;

pub trait LearnerCourseDetailUseCase: Send + Sync {
    fn get_learner_course_detail(
        &self,
        query: LearnerCourseDetailQuery,
    ) -> BoxFuture<'_, Result<LearnerCourseDetailOutput, LearnerCourseCatalogError>>;
}
