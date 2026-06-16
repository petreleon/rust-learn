use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::application::content::inspect_processing_history::{
    ContentProcessingHistoryOutput, ContentProcessingJobOutput,
};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ContentProcessingHistoryResponse {
    pub content_id: i32,
    pub object_key: Option<String>,
    pub jobs: Vec<ContentProcessingJobResponse>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ContentProcessingJobResponse {
    pub id: i64,
    pub status: String,
    pub attempts: i32,
    pub last_error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<ContentProcessingHistoryOutput> for ContentProcessingHistoryResponse {
    fn from(output: ContentProcessingHistoryOutput) -> Self {
        Self {
            content_id: output.content_id,
            object_key: output.object_key,
            jobs: output.jobs.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<ContentProcessingJobOutput> for ContentProcessingJobResponse {
    fn from(job: ContentProcessingJobOutput) -> Self {
        Self {
            id: job.id,
            status: job.status,
            attempts: job.attempts,
            last_error: job.last_error,
            created_at: job.created_at,
            updated_at: job.updated_at,
        }
    }
}
