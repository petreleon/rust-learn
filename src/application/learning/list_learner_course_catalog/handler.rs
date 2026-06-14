use crate::application::learning::learner_course_catalog::LearnerCourseCatalogError;
use crate::application::learning::list_learner_course_catalog::{
    LearnerCourseCatalogListStore, LearnerCourseCatalogOutput, LearnerCourseCatalogQuery,
};

pub async fn list_learner_course_catalog(
    store: &mut impl LearnerCourseCatalogListStore,
    query: LearnerCourseCatalogQuery,
) -> Result<LearnerCourseCatalogOutput, LearnerCourseCatalogError> {
    store.list_catalog(query).await
}
