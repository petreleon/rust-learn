#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseUpdateOutput {
    pub id: i32,
    pub title: String,
    pub lifecycle_status: String,
    pub description: Option<String>,
    pub topics: Option<String>,
    pub prerequisites: Option<String>,
}
