use futures::future::BoxFuture;

use crate::application::learning::assign_course_role::{
    CourseRoleAssignmentCommand, CourseRoleAssignmentError, CourseRoleAssignmentOutput,
};

pub trait CourseRoleAssignmentUseCase: Send + Sync {
    fn assign_course_role(
        &self,
        command: CourseRoleAssignmentCommand,
    ) -> BoxFuture<'_, Result<CourseRoleAssignmentOutput, CourseRoleAssignmentError>>;
}
