use futures::future::BoxFuture;

use crate::application::learning::learner_course_catalog::LearnerCourseCatalogError;
use crate::application::learning::list_learner_course_catalog::{
    LearnerCourseCatalogOutput, LearnerCourseCatalogQuery,
};

pub trait LearnerCourseCatalogListStore {
    fn list_catalog(
        &mut self,
        query: LearnerCourseCatalogQuery,
    ) -> BoxFuture<'_, Result<LearnerCourseCatalogOutput, LearnerCourseCatalogError>>;
}
