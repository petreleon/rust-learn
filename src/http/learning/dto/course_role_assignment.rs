use serde::Deserialize;

use crate::application::learning::assign_course_role::CourseRoleAssignmentCommand;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct AssignCourseRoleRequest {
    pub role_name: String,
}

impl AssignCourseRoleRequest {
    pub fn into_command(
        self,
        actor_user_id: i32,
        course_id: i32,
        target_user_id: i32,
    ) -> CourseRoleAssignmentCommand {
        CourseRoleAssignmentCommand {
            actor_user_id,
            course_id,
            target_user_id,
            role_name: self.role_name,
        }
    }
}
