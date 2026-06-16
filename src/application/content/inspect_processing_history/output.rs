use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentProcessingHistoryOutput {
    pub content_id: i32,
    pub object_key: Option<String>,
    pub jobs: Vec<ContentProcessingJobOutput>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentProcessingJobOutput {
    pub id: i64,
    pub status: String,
    pub attempts: i32,
    pub last_error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
