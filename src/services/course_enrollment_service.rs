use crate::config::constants::permissions::Permissions;
use crate::db::schema::{course_join_requests, courses, courses_organizations, user_role_course};
use crate::models::course_join_request::{
    CourseJoinRequest, NewCourseJoinRequest, COURSE_JOIN_STATUS_APPROVED,
    COURSE_JOIN_STATUS_PENDING, COURSE_JOIN_STATUS_REJECTED,
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
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CourseJoinDecisionRequest {
    pub status: String,
    pub decision_reason: Option<String>,
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
        .filter(course_join_requests::status.eq(COURSE_JOIN_STATUS_PENDING))
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

            if existing.status != COURSE_JOIN_STATUS_PENDING {
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

async fn ensure_course_exists(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<(), CourseEnrollmentError> {
    courses::table
        .find(course_id)
        .select(courses::id)
        .first::<i32>(conn)
        .await?;
    Ok(())
}

async fn ensure_join_request_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
) -> Result<(), CourseEnrollmentError> {
    for permission in [
        Permissions::REQUEST_JOIN_COURSE.to_string(),
        Permissions::JOIN_COURSE.to_string(),
    ] {
        if user_has_permission_for_course_context(conn, user_id, course_id, &permission).await? {
            return Ok(());
        }
    }

    Err(CourseEnrollmentError::PermissionDenied(
        Permissions::REQUEST_JOIN_COURSE.to_string(),
    ))
}

async fn ensure_join_approval_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
) -> Result<(), CourseEnrollmentError> {
    for permission in [
        Permissions::APPROVE_COURSE_JOIN_REQUESTS.to_string(),
        Permissions::MANAGE_COURSE_ENROLLMENTS.to_string(),
    ] {
        if user_has_permission_for_course_context(conn, user_id, course_id, &permission).await? {
            return Ok(());
        }
    }

    Err(CourseEnrollmentError::PermissionDenied(
        Permissions::APPROVE_COURSE_JOIN_REQUESTS.to_string(),
    ))
}

async fn user_has_permission_for_course_context(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    permission: &str,
) -> Result<bool, CourseEnrollmentError> {
    if user_permission_course_request(conn, user_id, course_id, permission).await? {
        return Ok(true);
    }

    if user_permission_platform_request(conn, user_id, permission).await? {
        return Ok(true);
    }

    let organization_ids = courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .select(courses_organizations::organization_id)
        .load::<i32>(conn)
        .await?;

    for organization_id in organization_ids {
        if user_permission_organization_request(conn, user_id, organization_id, permission).await? {
            return Ok(true);
        }
    }

    Ok(false)
}

async fn has_course_student_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
) -> Result<bool, CourseEnrollmentError> {
    let student_role_id = CourseRole::find_by_name("STUDENT", conn).await?;
    diesel::select(exists(
        user_role_course::table
            .filter(user_role_course::user_id.eq(user_id))
            .filter(user_role_course::course_id.eq(course_id))
            .filter(user_role_course::course_role_id.eq(student_role_id)),
    ))
    .get_result(conn)
    .await
    .map_err(CourseEnrollmentError::from)
}

async fn assign_student_course_role_if_missing(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
) -> Result<(), CourseEnrollmentError> {
    if has_course_student_role(conn, user_id, course_id).await? {
        return Ok(());
    }

    let student_role_id = CourseRole::find_by_name("STUDENT", conn).await?;
    UserRoleCourse::assign(conn, user_id, course_id, student_role_id).await?;
    Ok(())
}

fn normalize_join_decision(status: &str) -> Result<String, CourseEnrollmentError> {
    let normalized = status.trim().to_ascii_lowercase();
    match normalized.as_str() {
        COURSE_JOIN_STATUS_APPROVED | COURSE_JOIN_STATUS_REJECTED => Ok(normalized),
        _ => Err(CourseEnrollmentError::InvalidStatus(
            "unsupported course join decision status".to_string(),
        )),
    }
}
