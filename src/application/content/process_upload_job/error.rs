#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessUploadJobError {
    ChapterNotFound,
    ContentNotFound,
    NonVideoContent,
    MissingObjectKey,
    InvalidObjectKey,
    ChapterLookupFailed(String),
    ContentLookupFailed(String),
    JobQueueFailed { object_key: String, message: String },
}
