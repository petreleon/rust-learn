use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::learning::learner_course_catalog::{
    LearnerCourseCatalogError, LearnerCourseEnrollmentSummaryOutput,
};
use crate::db::schema::{course_join_requests, course_roles, user_role_course};
use crate::models::course::COURSE_STATUS_PUBLISHED;
use crate::models::course_join_request::{
    CourseJoinRequest, COURSE_JOIN_STATUS_APPROVED, COURSE_JOIN_STATUS_PENDING,
    COURSE_JOIN_STATUS_REJECTED, COURSE_JOIN_STATUS_WAITLISTED,
};

pub async fn build_learner_course_enrollment(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    lifecycle_status: &str,
    can_request_join: bool,
) -> Result<LearnerCourseEnrollmentSummaryOutput, LearnerCourseCatalogError> {
    let roles = load_actor_course_roles(conn, actor_user_id, course_id).await?;
    let latest_request = course_join_requests::table
        .filter(course_join_requests::course_id.eq(course_id))
        .filter(course_join_requests::requester_user_id.eq(actor_user_id))
        .order(course_join_requests::updated_at.desc())
        .first::<CourseJoinRequest>(conn)
        .await
        .optional()
        .map_err(map_learning_error)?;

    Ok(build_enrollment_summary(
        latest_request,
        roles,
        lifecycle_status,
        can_request_join,
    ))
}

async fn load_actor_course_roles(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> Result<Vec<String>, LearnerCourseCatalogError> {
    let mut roles = user_role_course::table
        .inner_join(
            course_roles::table
                .on(user_role_course::course_role_id.eq(course_roles::id.nullable())),
        )
        .filter(user_role_course::user_id.eq(actor_user_id))
        .filter(user_role_course::course_id.eq(course_id))
        .order(course_roles::name.asc())
        .select(course_roles::name)
        .load::<String>(conn)
        .await
        .map_err(map_learning_error)?;

    roles.dedup();
    Ok(roles)
}

fn build_enrollment_summary(
    latest_request: Option<CourseJoinRequest>,
    roles: Vec<String>,
    lifecycle_status: &str,
    can_request_join: bool,
) -> LearnerCourseEnrollmentSummaryOutput {
    if roles.iter().any(|role| role == "STUDENT") {
        return enrollment_summary(
            "enrolled",
            latest_request.as_ref().map(|request| request.id),
            false,
            Some("You already have course access."),
            roles,
        );
    }

    if let Some(request) = latest_request {
        return match request.status.as_str() {
            COURSE_JOIN_STATUS_PENDING | COURSE_JOIN_STATUS_WAITLISTED => enrollment_summary(
                request.status.as_str(),
                Some(request.id),
                false,
                Some("Your join request is waiting for review."),
                roles,
            ),
            COURSE_JOIN_STATUS_REJECTED => enrollment_summary(
                COURSE_JOIN_STATUS_REJECTED,
                Some(request.id),
                can_request_join,
                Some("Your previous join request was rejected; you can request again if access is open."),
                roles,
            ),
            COURSE_JOIN_STATUS_APPROVED => enrollment_summary(
                "enrolled",
                Some(request.id),
                false,
                Some("Your join request was approved."),
                roles,
            ),
            _ => unavailable_or_available_summary(lifecycle_status, can_request_join, roles),
        };
    }

    unavailable_or_available_summary(lifecycle_status, can_request_join, roles)
}

fn unavailable_or_available_summary(
    lifecycle_status: &str,
    can_request_join: bool,
    roles: Vec<String>,
) -> LearnerCourseEnrollmentSummaryOutput {
    if lifecycle_status != COURSE_STATUS_PUBLISHED {
        return enrollment_summary(
            "unavailable",
            None,
            false,
            Some("This course is not published for learner enrollment."),
            roles,
        );
    }

    enrollment_summary(
        "available",
        None,
        can_request_join,
        Some(if can_request_join {
            "Enrollment can be requested."
        } else {
            "Your account cannot request course enrollment yet."
        }),
        roles,
    )
}

fn enrollment_summary(
    state: &str,
    request_id: Option<i64>,
    can_request_join: bool,
    reason: Option<&str>,
    roles: Vec<String>,
) -> LearnerCourseEnrollmentSummaryOutput {
    LearnerCourseEnrollmentSummaryOutput {
        state: state.to_string(),
        request_id,
        can_request_join,
        reason: reason.map(str::to_string),
        roles,
    }
}

fn map_learning_error(error: diesel::result::Error) -> LearnerCourseCatalogError {
    match error {
        diesel::result::Error::NotFound => LearnerCourseCatalogError::NotFound,
        other => LearnerCourseCatalogError::Database(other.to_string()),
    }
}
