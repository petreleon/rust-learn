use futures::future::BoxFuture;

use crate::application::rewards::list_reward_history::{
    StudentRewardHistoryEntry, StudentRewardHistoryError, StudentRewardHistoryQuery,
};

pub trait StudentRewardHistoryUseCase: Send + Sync {
    fn list_student_reward_history(
        &self,
        actor_user_id: i32,
        query: StudentRewardHistoryQuery,
    ) -> BoxFuture<'_, Result<Vec<StudentRewardHistoryEntry>, StudentRewardHistoryError>>;
}
