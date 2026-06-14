#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChapterOutput {
    pub id: i32,
    pub course_id: i32,
    pub title: String,
    pub order: i32,
}
