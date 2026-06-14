#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateChapterCommand {
    pub course_id: i32,
    pub title: String,
    pub order: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateChapterCommand {
    pub title: Option<String>,
    pub order: Option<i32>,
}
