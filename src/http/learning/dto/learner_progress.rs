use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::learning::learner_progress::{
    LearnerProgressOutput, SaveLearnerProgressCommand,
};

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct SaveProgressRequest {
    pub content_id: i32,
}

impl SaveProgressRequest {
    pub fn into_command(self, actor_user_id: i32, course_id: i32) -> SaveLearnerProgressCommand {
        SaveLearnerProgressCommand {
            actor_user_id,
            course_id,
            content_id: self.content_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LearnerProgressResponse {
    pub id: i32,
    pub user_id: i32,
    pub course_id: i32,
    pub content_id: i32,
    pub viewed_at: DateTime<Utc>,
}

impl From<LearnerProgressOutput> for LearnerProgressResponse {
    fn from(progress: LearnerProgressOutput) -> Self {
        Self {
            id: progress.id,
            user_id: progress.user_id,
            course_id: progress.course_id,
            content_id: progress.content_id,
            viewed_at: progress.viewed_at,
        }
    }
}
