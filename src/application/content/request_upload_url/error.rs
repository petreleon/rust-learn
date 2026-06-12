#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentUploadUrlError {
    ChapterNotFound,
    MissingContentType,
    Database(String),
    BucketPrepareFailed(String),
    PresignFailed(String),
}
