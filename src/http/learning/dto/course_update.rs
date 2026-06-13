use serde::Deserialize;

use crate::application::learning::update_course::CourseUpdateCommand;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct CourseUpdateRequest {
    pub title: Option<String>,
    pub description: Option<Option<String>>,
    pub topics: Option<Option<String>>,
    pub prerequisites: Option<Option<String>>,
}

impl CourseUpdateRequest {
    pub fn into_command(self, actor_user_id: i32, course_id: i32) -> CourseUpdateCommand {
        CourseUpdateCommand {
            actor_user_id,
            course_id,
            title: self.title,
            description: self.description,
            topics: self.topics,
            prerequisites: self.prerequisites,
        }
    }
}
