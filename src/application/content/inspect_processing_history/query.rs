#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentProcessingHistoryQuery {
    pub course_id: i32,
    pub chapter_id: i32,
    pub content_id: i32,
}
