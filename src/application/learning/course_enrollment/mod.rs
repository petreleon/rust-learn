mod command;
mod error;
mod handler;
mod output;
mod service;
mod store;
mod validation;

#[cfg(test)]
mod tests;

pub use command::{
    DecideCourseJoinCommand, RemoveCourseEnrollmentCommand, RequestCourseJoinCommand,
};
pub use error::CourseEnrollmentError;
pub use handler::{decide_course_join_request, remove_course_enrollment, request_course_join};
pub use output::{
    CourseEnrollmentRemovalOutput, CourseJoinDecisionOutput, CourseJoinRequestOutput,
    EnrollmentNotification,
};
pub use service::CourseEnrollmentUseCase;
pub use store::CourseEnrollmentStore;
