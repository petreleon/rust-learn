use chrono::{DateTime, Utc};
use futures::future::BoxFuture;

use crate::application::learning::course_enrollment::{
    CourseEnrollmentError, CourseJoinRequestOutput,
};

pub trait CourseEnrollmentStore {
    fn course_exists(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<bool, CourseEnrollmentError>>;

    fn has_course_context_permission(
        &mut self,
        user_id: i32,
        course_id: i32,
        permission: &str,
    ) -> BoxFuture<'_, Result<bool, CourseEnrollmentError>>;

    fn has_student_role(
        &mut self,
        user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<bool, CourseEnrollmentError>>;

    fn open_join_request(
        &mut self,
        course_id: i32,
        requester_user_id: i32,
    ) -> BoxFuture<'_, Result<Option<CourseJoinRequestOutput>, CourseEnrollmentError>>;

    fn create_join_request(
        &mut self,
        course_id: i32,
        requester_user_id: i32,
        status: String,
    ) -> BoxFuture<'_, Result<CourseJoinRequestOutput, CourseEnrollmentError>>;

    fn join_request(
        &mut self,
        request_id: i64,
    ) -> BoxFuture<'_, Result<Option<CourseJoinRequestOutput>, CourseEnrollmentError>>;

    fn update_join_request_decision(
        &mut self,
        request_id: i64,
        status: String,
        reviewer_user_id: i32,
        decision_reason: Option<String>,
        decided_at: DateTime<Utc>,
    ) -> BoxFuture<'_, Result<CourseJoinRequestOutput, CourseEnrollmentError>>;

    fn assign_student_role_if_missing(
        &mut self,
        user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<(), CourseEnrollmentError>>;

    fn active_completion_terms_capacity(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Option<i32>, CourseEnrollmentError>>;

    fn enrolled_student_count(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<i64, CourseEnrollmentError>>;

    fn remove_student_role(
        &mut self,
        user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<bool, CourseEnrollmentError>>;

    fn course_title(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Option<String>, CourseEnrollmentError>>;
}
