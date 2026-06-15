use diesel::dsl::{exists, select};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};
use crate::application::learning::course_enrollment::{
    CourseEnrollmentError, CourseJoinRequestOutput,
};
use crate::db::schema::{course_roles, courses_organizations, user_role_course};
use crate::infra::postgres::access_control::permission_checks;
use crate::infra::postgres::models::course_join_request::CourseJoinRequest;

const COURSE_ROLE_STUDENT: &str = "STUDENT";

impl From<diesel::result::Error> for CourseEnrollmentError {
    fn from(error: diesel::result::Error) -> Self {
        map_enrollment_error(error)
    }
}

impl From<CourseJoinRequest> for CourseJoinRequestOutput {
    fn from(request: CourseJoinRequest) -> Self {
        Self {
            id: request.id,
            course_id: request.course_id,
            requester_user_id: request.requester_user_id,
            status: request.status,
            reviewer_user_id: request.reviewer_user_id,
            decision_reason: request.decision_reason,
            created_at: request.created_at,
            updated_at: request.updated_at,
            decided_at: request.decided_at,
        }
    }
}

pub async fn has_course_context_permission(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    permission: &str,
) -> Result<bool, CourseEnrollmentError> {
    if permission_checks::can(
        conn,
        AccessActor::user(user_id),
        AccessAction::permission(permission),
        AccessScope::course(course_id),
    )
    .await?
        || permission_checks::can(
            conn,
            AccessActor::user(user_id),
            AccessAction::permission(permission),
            AccessScope::platform(),
        )
        .await?
    {
        return Ok(true);
    }

    for organization_id in course_organization_ids(conn, course_id).await? {
        if permission_checks::can(
            conn,
            AccessActor::user(user_id),
            AccessAction::permission(permission),
            AccessScope::organization(organization_id),
        )
        .await?
        {
            return Ok(true);
        }
    }

    Ok(false)
}

pub async fn has_student_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
) -> Result<bool, CourseEnrollmentError> {
    let role_id = student_role_id(conn).await?;
    select(exists(
        user_role_course::table
            .filter(user_role_course::user_id.eq(user_id))
            .filter(user_role_course::course_id.eq(course_id))
            .filter(user_role_course::course_role_id.eq(role_id)),
    ))
    .get_result(conn)
    .await
    .map_err(map_enrollment_error)
}

pub async fn assign_student_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
) -> Result<(), CourseEnrollmentError> {
    let role_id = student_role_id(conn).await?;
    diesel::insert_into(user_role_course::table)
        .values((
            user_role_course::user_id.eq(user_id),
            user_role_course::course_id.eq(course_id),
            user_role_course::course_role_id.eq(role_id),
        ))
        .execute(conn)
        .await
        .map(|_| ())
        .map_err(map_enrollment_error)
}

pub async fn student_role_id(conn: &mut AsyncPgConnection) -> Result<i32, CourseEnrollmentError> {
    course_roles::table
        .filter(course_roles::name.eq(COURSE_ROLE_STUDENT))
        .select(course_roles::id)
        .first(conn)
        .await
        .map_err(map_enrollment_error)
}

pub fn map_enrollment_error(error: diesel::result::Error) -> CourseEnrollmentError {
    match error {
        diesel::result::Error::NotFound => CourseEnrollmentError::NotFound,
        other => CourseEnrollmentError::Database(other.to_string()),
    }
}

async fn course_organization_ids(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<Vec<i32>, CourseEnrollmentError> {
    courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .select(courses_organizations::organization_id)
        .load(conn)
        .await
        .map_err(map_enrollment_error)
}
