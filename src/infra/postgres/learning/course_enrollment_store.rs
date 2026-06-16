use chrono::{DateTime, Utc};
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::learning::course_enrollment::{
    CourseEnrollmentError, CourseEnrollmentStore, CourseJoinRequestOutput,
};
use crate::infra::postgres::learning::course_enrollment_join_queries;
use crate::infra::postgres::learning::course_enrollment_queries::{
    active_completion_terms_capacity, assign_student_role, enrolled_student_count,
    has_course_context_permission, has_student_role,
};

pub struct PostgresCourseEnrollmentStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresCourseEnrollmentStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl CourseEnrollmentStore for PostgresCourseEnrollmentStore<'_> {
    fn course_exists(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<bool, CourseEnrollmentError>> {
        async move { course_enrollment_join_queries::course_exists(self.conn, course_id).await }
            .boxed()
    }

    fn has_course_context_permission(
        &mut self,
        user_id: i32,
        course_id: i32,
        permission: &str,
    ) -> BoxFuture<'_, Result<bool, CourseEnrollmentError>> {
        let permission = permission.to_string();
        async move { has_course_context_permission(self.conn, user_id, course_id, &permission).await }
            .boxed()
    }

    fn has_student_role(
        &mut self,
        user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<bool, CourseEnrollmentError>> {
        async move { has_student_role(self.conn, user_id, course_id).await }.boxed()
    }

    fn open_join_request(
        &mut self,
        course_id: i32,
        requester_user_id: i32,
    ) -> BoxFuture<'_, Result<Option<CourseJoinRequestOutput>, CourseEnrollmentError>> {
        async move {
            course_enrollment_join_queries::open_join_request(
                self.conn,
                course_id,
                requester_user_id,
            )
            .await
        }
        .boxed()
    }

    fn create_join_request(
        &mut self,
        course_id: i32,
        requester_user_id: i32,
        status: String,
    ) -> BoxFuture<'_, Result<CourseJoinRequestOutput, CourseEnrollmentError>> {
        async move {
            course_enrollment_join_queries::create_join_request(
                self.conn,
                course_id,
                requester_user_id,
                status,
            )
            .await
        }
        .boxed()
    }

    fn join_request(
        &mut self,
        request_id: i64,
    ) -> BoxFuture<'_, Result<Option<CourseJoinRequestOutput>, CourseEnrollmentError>> {
        async move { course_enrollment_join_queries::join_request(self.conn, request_id).await }
            .boxed()
    }

    fn update_join_request_decision(
        &mut self,
        request_id: i64,
        status: String,
        reviewer_user_id: i32,
        decision_reason: Option<String>,
        decided_at: DateTime<Utc>,
    ) -> BoxFuture<'_, Result<CourseJoinRequestOutput, CourseEnrollmentError>> {
        async move {
            course_enrollment_join_queries::update_join_request_decision(
                self.conn,
                request_id,
                status,
                reviewer_user_id,
                decision_reason,
                decided_at,
            )
            .await
        }
        .boxed()
    }

    fn assign_student_role_if_missing(
        &mut self,
        user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<(), CourseEnrollmentError>> {
        async move {
            if !has_student_role(self.conn, user_id, course_id).await? {
                assign_student_role(self.conn, user_id, course_id).await?;
            }
            Ok(())
        }
        .boxed()
    }

    fn active_completion_terms_capacity(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Option<i32>, CourseEnrollmentError>> {
        async move { active_completion_terms_capacity(self.conn, course_id).await }.boxed()
    }

    fn enrolled_student_count(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<i64, CourseEnrollmentError>> {
        async move { enrolled_student_count(self.conn, course_id).await }.boxed()
    }

    fn remove_student_role(
        &mut self,
        user_id: i32,
        course_id: i32,
    ) -> BoxFuture<'_, Result<bool, CourseEnrollmentError>> {
        async move {
            course_enrollment_join_queries::remove_student_role(self.conn, user_id, course_id).await
        }
        .boxed()
    }

    fn course_title(
        &mut self,
        course_id: i32,
    ) -> BoxFuture<'_, Result<Option<String>, CourseEnrollmentError>> {
        async move { course_enrollment_join_queries::course_title(self.conn, course_id).await }
            .boxed()
    }
}
