use futures::future::BoxFuture;

use crate::application::learning::get_learner_course_learning::{
    LearnerCourseLearningError, LearnerCourseLearningOutput, LearnerCourseLearningQuery,
};

pub trait LearnerCourseLearningUseCase: Send + Sync {
    fn get_learner_course_learning(
        &self,
        query: LearnerCourseLearningQuery,
    ) -> BoxFuture<'_, Result<LearnerCourseLearningOutput, LearnerCourseLearningError>>;
}
