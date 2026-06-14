use crate::application::learning::get_learner_course_learning::{
    LearnerCourseLearningError, LearnerCourseLearningOutput, LearnerCourseLearningQuery,
    LearnerCourseLearningStore,
};

pub async fn get_learner_course_learning(
    store: &mut impl LearnerCourseLearningStore,
    query: LearnerCourseLearningQuery,
) -> Result<LearnerCourseLearningOutput, LearnerCourseLearningError> {
    store.get_learning(query).await
}
