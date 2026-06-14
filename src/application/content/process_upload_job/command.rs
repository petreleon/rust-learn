#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessUploadJobCommand {
    pub course_id: i32,
    pub chapter_id: i32,
    pub content_id: i32,
    pub user_id: i32,
}
