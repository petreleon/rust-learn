use serde::Deserialize;

use crate::application::learning::create_course::CourseCreationCommand;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct CreateCourseRequest {
    pub title: String,
    pub organization_ids: Vec<i32>,
}

impl CreateCourseRequest {
    pub fn into_command(self, actor_user_id: i32) -> CourseCreationCommand {
        CourseCreationCommand {
            actor_user_id,
            title: self.title,
            organization_ids: self.organization_ids,
        }
    }
}
