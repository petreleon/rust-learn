use crate::config::constants::permissions::Permissions;
use crate::db::schema::{course_join_requests, courses, courses_organizations, user_role_course};
use crate::models::course_join_request::{
    CourseJoinRequest, NewCourseJoinRequest, COURSE_JOIN_STATUS_APPROVED,
    COURSE_JOIN_STATUS_PENDING, COURSE_JOIN_STATUS_REJECTED, COURSE_JOIN_STATUS_WAITLISTED,
};
use crate::models::role::CourseRole;
use crate::models::user_role_course::UserRoleCourse;
use crate::repositories::course_repository::user_permission_course_request;
use crate::repositories::organization_repository::user_permission_organization_request;
use crate::repositories::platform_repository::user_permission_platform_request;
use chrono::Utc;
use diesel::dsl::exists;
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct CourseJoinDecisionRequest {
    pub status: String,
    pub decision_reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CourseEnrollmentRemovalResponse {
    pub course_id: i32,
    pub user_id: i32,
    pub removed: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CourseEnrollmentError {
    PermissionDenied(String),
    InvalidStatus(String),
    NotFound,
    Database(String),
}

impl From<diesel::result::Error> for CourseEnrollmentError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => CourseEnrollmentError::NotFound,
            other => CourseEnrollmentError::Database(other.to_string()),
        }
    }
}

pub async fn request_course_join(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> Result<CourseJoinRequest, CourseEnrollmentError> {
    ensure_course_exists(conn, course_id).await?;
    ensure_join_request_permission(conn, actor_user_id, course_id).await?;

    if has_course_student_role(conn, actor_user_id, course_id).await? {
        return Err(CourseEnrollmentError::InvalidStatus(
            "user is already enrolled in this course".to_string(),
        ));
    }

    if let Some(existing) = course_join_requests::table
        .filter(course_join_requests::course_id.eq(course_id))
        .filter(course_join_requests::requester_user_id.eq(actor_user_id))
        .filter(
            course_join_requests::status
                .eq_any([COURSE_JOIN_STATUS_PENDING, COURSE_JOIN_STATUS_WAITLISTED]),
        )
        .first::<CourseJoinRequest>(conn)
        .await
        .optional()?
    {
        return Ok(existing);
    }

    let new_request = NewCourseJoinRequest {
        course_id,
        requester_user_id: actor_user_id,
        status: COURSE_JOIN_STATUS_PENDING.to_string(),
    };

    diesel::insert_into(course_join_requests::table)
        .values(&new_request)
        .get_result::<CourseJoinRequest>(conn)
        .await
        .map_err(CourseEnrollmentError::from)
}

pub async fn decide_course_join_request(
    conn: &mut AsyncPgConnection,
    reviewer_user_id: i32,
    course_id: i32,
    request_id: i64,
    request: CourseJoinDecisionRequest,
) -> Result<CourseJoinRequest, CourseEnrollmentError> {
    let target_status = normalize_join_decision(&request.status)?;

    conn.transaction::<_, CourseEnrollmentError, _>(|conn| {
        Box::pin(async move {
            let existing = course_join_requests::table
                .find(request_id)
                .first::<CourseJoinRequest>(conn)
                .await?;

            if existing.course_id != course_id {
                return Err(CourseEnrollmentError::NotFound);
            }

            if existing.status == target_status {
                return Ok(existing);
            }

            if !matches!(
                existing.status.as_str(),
                COURSE_JOIN_STATUS_PENDING | COURSE_JOIN_STATUS_WAITLISTED
            ) {
                return Err(CourseEnrollmentError::InvalidStatus(
                    "course join request has already been decided".to_string(),
                ));
            }

            ensure_join_approval_permission(conn, reviewer_user_id, course_id).await?;

            let now = Utc::now();
            let updated = diesel::update(course_join_requests::table.find(request_id))
                .set((
                    course_join_requests::status.eq(target_status.clone()),
                    course_join_requests::reviewer_user_id.eq(Some(reviewer_user_id)),
                    course_join_requests::decision_reason.eq(request.decision_reason.clone()),
                    course_join_requests::updated_at.eq(now),
                    course_join_requests::decided_at.eq(Some(now)),
                ))
                .get_result::<CourseJoinRequest>(conn)
                .await?;

            if target_status == COURSE_JOIN_STATUS_APPROVED {
                assign_student_course_role_if_missing(
                    conn,
                    existing.requester_user_id,
                    existing.course_id,
                )
                .await?;
            }

            Ok(updated)
        })
    })
    .await
}
