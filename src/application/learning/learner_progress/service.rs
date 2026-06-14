use futures::future::BoxFuture;

use crate::application::learning::learner_progress::{
    LearnerProgressError, LearnerProgressOutput, SaveLearnerProgressCommand,
};

pub trait LearnerProgressUseCase: Send + Sync {
    fn save_progress(
        &self,
        command: SaveLearnerProgressCommand,
    ) -> BoxFuture<'_, Result<LearnerProgressOutput, LearnerProgressError>>;

    fn get_progress(
        &self,
        actor_user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Option<LearnerProgressOutput>, LearnerProgressError>>;
}
