use chrono::{DateTime, Utc};
use diesel::dsl::{exists, select};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::learning::course_enrollment::{
    CourseEnrollmentError, CourseJoinRequestOutput,
};
use crate::domain::learning::enrollment::status::{
    COURSE_JOIN_STATUS_PENDING, COURSE_JOIN_STATUS_WAITLISTED,
};
use crate::infra::postgres::learning::course_enrollment_queries::{
    map_enrollment_error, student_role_id,
};
use crate::infra::postgres::models::course_join_request::{
    CourseJoinRequest, NewCourseJoinRequest,
};
use crate::infra::postgres::schema::{course_join_requests, courses, user_role_course};

pub async fn course_exists(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<bool, CourseEnrollmentError> {
    select(exists(courses::table.find(course_id)))
        .get_result(conn)
        .await
        .map_err(map_enrollment_error)
}

pub async fn open_join_request(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    requester_user_id: i32,
) -> Result<Option<CourseJoinRequestOutput>, CourseEnrollmentError> {
    course_join_requests::table
        .filter(course_join_requests::course_id.eq(course_id))
        .filter(course_join_requests::requester_user_id.eq(requester_user_id))
        .filter(
            course_join_requests::status
                .eq_any([COURSE_JOIN_STATUS_PENDING, COURSE_JOIN_STATUS_WAITLISTED]),
        )
        .first::<CourseJoinRequest>(conn)
        .await
        .optional()
        .map(|request| request.map(CourseJoinRequestOutput::from))
        .map_err(map_enrollment_error)
}

pub async fn create_join_request(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    requester_user_id: i32,
    status: String,
) -> Result<CourseJoinRequestOutput, CourseEnrollmentError> {
    diesel::insert_into(course_join_requests::table)
        .values(NewCourseJoinRequest {
            course_id,
            requester_user_id,
            status,
        })
        .get_result::<CourseJoinRequest>(conn)
        .await
        .map(CourseJoinRequestOutput::from)
        .map_err(map_enrollment_error)
}

pub async fn join_request(
    conn: &mut AsyncPgConnection,
    request_id: i64,
) -> Result<Option<CourseJoinRequestOutput>, CourseEnrollmentError> {
    course_join_requests::table
        .find(request_id)
        .first::<CourseJoinRequest>(conn)
        .await
        .optional()
        .map(|request| request.map(CourseJoinRequestOutput::from))
        .map_err(map_enrollment_error)
}

pub async fn update_join_request_decision(
    conn: &mut AsyncPgConnection,
    request_id: i64,
    status: String,
    reviewer_user_id: i32,
    decision_reason: Option<String>,
    decided_at: DateTime<Utc>,
) -> Result<CourseJoinRequestOutput, CourseEnrollmentError> {
    diesel::update(course_join_requests::table.find(request_id))
        .set((
            course_join_requests::status.eq(status),
            course_join_requests::reviewer_user_id.eq(Some(reviewer_user_id)),
            course_join_requests::decision_reason.eq(decision_reason),
            course_join_requests::updated_at.eq(decided_at),
            course_join_requests::decided_at.eq(Some(decided_at)),
        ))
        .get_result::<CourseJoinRequest>(conn)
        .await
        .map(CourseJoinRequestOutput::from)
        .map_err(map_enrollment_error)
}

pub async fn remove_student_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
) -> Result<bool, CourseEnrollmentError> {
    let role_id = student_role_id(conn).await?;
    diesel::delete(
        user_role_course::table
            .filter(user_role_course::user_id.eq(Some(user_id)))
            .filter(user_role_course::course_id.eq(Some(course_id)))
            .filter(user_role_course::course_role_id.eq(Some(role_id))),
    )
    .execute(conn)
    .await
    .map(|count| count > 0)
    .map_err(map_enrollment_error)
}

pub async fn course_title(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<Option<String>, CourseEnrollmentError> {
    courses::table
        .find(course_id)
        .select(courses::title)
        .first::<String>(conn)
        .await
        .optional()
        .map_err(map_enrollment_error)
}
