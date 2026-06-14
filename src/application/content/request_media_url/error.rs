#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentMediaUrlError {
    ChapterNotFound,
    ContentNotFound,
    MissingObjectKey,
    InvalidObjectKey,
    Connection(String),
    ChapterLookupFailed(String),
    ContentLookupFailed(String),
    StorageClientInitFailed(String),
    PresignFailed { object_key: String, message: String },
}
