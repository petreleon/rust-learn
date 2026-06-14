#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentUploadUrlError {
    ChapterNotFound,
    MissingContentType,
    Connection(String),
    Database(String),
    StorageClientInitFailed(String),
    BucketPrepareFailed(String),
    PresignFailed(String),
}
