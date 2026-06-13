use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::learning::get_teacher_course_enrollment_workspace::TeacherCourseEnrollmentWorkspaceQuery;
use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardError;
use crate::application::learning::teacher_course_enrollment::{
    TeacherCourseJoinRequestItemOutput, TeacherCourseJoinRequestPageOutput,
};
use crate::db::schema::course_join_requests;
use crate::infra::postgres::learning::teacher_course_roster_queries;
use crate::models::course_join_request::{
    CourseJoinRequest, COURSE_JOIN_STATUS_PENDING, COURSE_JOIN_STATUS_WAITLISTED,
};

pub async fn load_teacher_course_join_request_page(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    query: &TeacherCourseEnrollmentWorkspaceQuery,
    can_manage_enrollments: bool,
) -> Result<TeacherCourseJoinRequestPageOutput, TeacherCourseDashboardError> {
    let mut count_query = course_join_requests::table.into_boxed();
    count_query = count_query.filter(course_join_requests::course_id.eq(course_id));
    count_query = apply_status_filter(count_query, query.status.as_deref());
    let total = count_query
        .count()
        .get_result::<i64>(conn)
        .await
        .map_err(map_dashboard_error)?;

    let mut list_query = course_join_requests::table.into_boxed();
    list_query = list_query.filter(course_join_requests::course_id.eq(course_id));
    list_query = apply_status_filter(list_query, query.status.as_deref());
    let rows = list_query
        .order(course_join_requests::updated_at.desc())
        .then_order_by(course_join_requests::id.asc())
        .limit(query.limit)
        .offset(query.offset)
        .load::<CourseJoinRequest>(conn)
        .await
        .map_err(map_dashboard_error)?;

    let mut requests = Vec::with_capacity(rows.len());
    for request in rows {
        requests.push(build_join_request_item(conn, request, can_manage_enrollments).await?);
    }

    Ok(TeacherCourseJoinRequestPageOutput {
        requests,
        total,
        limit: query.limit,
        offset: query.offset,
        status: query.status.clone(),
    })
}

fn apply_status_filter<'a>(
    query: course_join_requests::BoxedQuery<'a, diesel::pg::Pg>,
    status: Option<&'a str>,
) -> course_join_requests::BoxedQuery<'a, diesel::pg::Pg> {
    match status {
        Some("all") | None => query,
        Some("open") => query.filter(
            course_join_requests::status
                .eq_any([COURSE_JOIN_STATUS_PENDING, COURSE_JOIN_STATUS_WAITLISTED]),
        ),
        Some(status) => query.filter(course_join_requests::status.eq(status)),
    }
}

async fn build_join_request_item(
    conn: &mut AsyncPgConnection,
    request: CourseJoinRequest,
    can_manage_enrollments: bool,
) -> Result<TeacherCourseJoinRequestItemOutput, TeacherCourseDashboardError> {
    let requester = teacher_course_roster_queries::load_teacher_enrollment_user_summary(
        conn,
        request.requester_user_id,
    )
    .await?
    .ok_or_else(|| {
        TeacherCourseDashboardError::Database(format!(
            "missing requester user for join request {}",
            request.id
        ))
    })?;
    let reviewer = match request.reviewer_user_id {
        Some(user_id) => {
            teacher_course_roster_queries::load_teacher_enrollment_user_summary(conn, user_id)
                .await?
        }
        None => None,
    };
    let can_decide = can_manage_enrollments
        && matches!(
            request.status.as_str(),
            COURSE_JOIN_STATUS_PENDING | COURSE_JOIN_STATUS_WAITLISTED
        );

    Ok(TeacherCourseJoinRequestItemOutput {
        id: request.id,
        status: request.status,
        requester,
        reviewer,
        decision_reason: request.decision_reason,
        created_at: request.created_at,
        updated_at: request.updated_at,
        decided_at: request.decided_at,
        can_decide,
    })
}

fn map_dashboard_error(error: diesel::result::Error) -> TeacherCourseDashboardError {
    match error {
        diesel::result::Error::NotFound => TeacherCourseDashboardError::NotFound,
        other => TeacherCourseDashboardError::Database(other.to_string()),
    }
}
