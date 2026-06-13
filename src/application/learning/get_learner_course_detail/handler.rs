use crate::application::learning::get_learner_course_detail::{
    LearnerCourseDetailOutput, LearnerCourseDetailQuery, LearnerCourseDetailStore,
};
use crate::application::learning::learner_course_catalog::LearnerCourseCatalogError;

pub async fn get_learner_course_detail(
    store: &mut impl LearnerCourseDetailStore,
    query: LearnerCourseDetailQuery,
) -> Result<LearnerCourseDetailOutput, LearnerCourseCatalogError> {
    store.get_detail(query).await
}
