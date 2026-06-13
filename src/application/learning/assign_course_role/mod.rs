mod command;
mod error;
mod handler;
mod output;
mod service;
mod store;

#[cfg(test)]
mod tests;

pub use command::CourseRoleAssignmentCommand;
pub use error::CourseRoleAssignmentError;
pub use handler::assign_course_role;
pub use output::CourseRoleAssignmentOutput;
pub use service::CourseRoleAssignmentUseCase;
pub use store::CourseRoleAssignmentStore;
