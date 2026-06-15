use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::learning::teacher_course_dashboard::{
    record_teacher_course_reward_queue_status, TeacherCourseDashboardError,
    TeacherCourseRewardQueueSummaryOutput, TeacherCourseRosterSummaryOutput,
};
use crate::domain::learning::enrollment::status::{
    COURSE_JOIN_STATUS_PENDING, COURSE_JOIN_STATUS_WAITLISTED,
};
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::infra::postgres::schema::{
    course_join_requests, course_roles, reward_candidates, user_role_course,
};

pub async fn load_course_roster_summary(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<TeacherCourseRosterSummaryOutput, TeacherCourseDashboardError> {
    let enrolled_student_count = user_role_course::table
        .inner_join(
            course_roles::table
                .on(user_role_course::course_role_id.eq(course_roles::id.nullable())),
        )
        .filter(user_role_course::course_id.eq(course_id))
        .filter(course_roles::name.eq("STUDENT"))
        .count()
        .get_result::<i64>(conn)
        .await
        .map_err(map_dashboard_error)?;

    Ok(TeacherCourseRosterSummaryOutput {
        enrolled_student_count,
        pending_join_request_count: count_join_requests(
            conn,
            course_id,
            COURSE_JOIN_STATUS_PENDING,
        )
        .await?,
        waitlisted_join_request_count: count_join_requests(
            conn,
            course_id,
            COURSE_JOIN_STATUS_WAITLISTED,
        )
        .await?,
    })
}

pub async fn load_course_reward_queue_summary(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<TeacherCourseRewardQueueSummaryOutput, TeacherCourseDashboardError> {
    let mut summary = TeacherCourseRewardQueueSummaryOutput {
        pending_teacher_count: 0,
        teacher_approved_count: 0,
        failed_count: 0,
    };
    for (status, count) in load_reward_candidate_status_counts(conn, course_id).await? {
        if let Ok(status) = RewardCandidateStatus::parse(&status) {
            record_teacher_course_reward_queue_status(&mut summary, status, count);
        }
    }
    Ok(summary)
}

async fn count_join_requests(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    status: &str,
) -> Result<i64, TeacherCourseDashboardError> {
    course_join_requests::table
        .filter(course_join_requests::course_id.eq(course_id))
        .filter(course_join_requests::status.eq(status))
        .count()
        .get_result::<i64>(conn)
        .await
        .map_err(map_dashboard_error)
}

async fn load_reward_candidate_status_counts(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<Vec<(String, i64)>, TeacherCourseDashboardError> {
    reward_candidates::table
        .filter(reward_candidates::course_id.eq(course_id))
        .group_by(reward_candidates::status)
        .select((reward_candidates::status, diesel::dsl::count_star()))
        .load::<(String, i64)>(conn)
        .await
        .map_err(map_dashboard_error)
}

fn map_dashboard_error(error: diesel::result::Error) -> TeacherCourseDashboardError {
    match error {
        diesel::result::Error::NotFound => TeacherCourseDashboardError::NotFound,
        other => TeacherCourseDashboardError::Database(other.to_string()),
    }
}
