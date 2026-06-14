use futures::future::BoxFuture;

use crate::application::learning::course_enrollment::{
    CourseEnrollmentError, CourseEnrollmentRemovalOutput, CourseJoinDecisionOutput,
    CourseJoinRequestOutput, DecideCourseJoinCommand, RemoveCourseEnrollmentCommand,
    RequestCourseJoinCommand,
};

pub trait CourseEnrollmentUseCase: Send + Sync {
    fn request_course_join(
        &self,
        command: RequestCourseJoinCommand,
    ) -> BoxFuture<'_, Result<CourseJoinRequestOutput, CourseEnrollmentError>>;

    fn decide_course_join_request(
        &self,
        command: DecideCourseJoinCommand,
    ) -> BoxFuture<'_, Result<CourseJoinDecisionOutput, CourseEnrollmentError>>;

    fn remove_course_enrollment(
        &self,
        command: RemoveCourseEnrollmentCommand,
    ) -> BoxFuture<'_, Result<CourseEnrollmentRemovalOutput, CourseEnrollmentError>>;
}
