use futures::future::BoxFuture;

use crate::application::learning::get_learner_course_learning::{
    LearnerCourseLearningError, LearnerCourseLearningOutput, LearnerCourseLearningQuery,
};

pub trait LearnerCourseLearningStore {
    fn get_learning(
        &mut self,
        query: LearnerCourseLearningQuery,
    ) -> BoxFuture<'_, Result<LearnerCourseLearningOutput, LearnerCourseLearningError>>;
}
