#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestUploadUrlCommand {
    pub course_id: i32,
    pub chapter_id: i32,
    pub filename: String,
    pub content_type: String,
}
