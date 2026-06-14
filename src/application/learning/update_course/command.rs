#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseUpdateCommand {
    pub actor_user_id: i32,
    pub course_id: i32,
    pub title: Option<String>,
    pub description: Option<Option<String>>,
    pub topics: Option<Option<String>>,
    pub prerequisites: Option<Option<String>>,
}

impl CourseUpdateCommand {
    pub fn patch(&self) -> CourseUpdatePatch {
        CourseUpdatePatch {
            title: self.title.clone(),
            description: self.description.clone(),
            topics: self.topics.clone(),
            prerequisites: self.prerequisites.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseUpdatePatch {
    pub title: Option<String>,
    pub description: Option<Option<String>>,
    pub topics: Option<Option<String>>,
    pub prerequisites: Option<Option<String>>,
}
