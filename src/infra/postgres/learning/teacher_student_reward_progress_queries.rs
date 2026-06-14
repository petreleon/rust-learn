use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde_json::Value;

use crate::application::learning::get_teacher_course_students::{
    TeacherStudentRewardCandidateSummaryOutput, TeacherStudentRewardProgressSummaryOutput,
};
use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardError;
use crate::db::schema::reward_candidates;
use crate::domain::rewards::candidate::status::{
    REWARD_STATUS_COMPLETED, REWARD_STATUS_FAILED, REWARD_STATUS_PENDING_TEACHER_APPROVAL,
    REWARD_STATUS_TEACHER_APPROVED, REWARD_STATUS_TEACHER_REJECTED,
};

pub async fn load_teacher_student_reward_progress(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    student_user_id: i32,
) -> Result<TeacherStudentRewardProgressSummaryOutput, TeacherCourseDashboardError> {
    Ok(TeacherStudentRewardProgressSummaryOutput {
        reward_candidate_count: count_student_reward_candidates(
            conn,
            course_id,
            student_user_id,
            None,
        )
        .await?,
        pending_teacher_count: count_student_reward_candidates(
            conn,
            course_id,
            student_user_id,
            Some(REWARD_STATUS_PENDING_TEACHER_APPROVAL),
        )
        .await?,
        teacher_approved_count: count_student_reward_candidates(
            conn,
            course_id,
            student_user_id,
            Some(REWARD_STATUS_TEACHER_APPROVED),
        )
        .await?,
        teacher_rejected_count: count_student_reward_candidates(
            conn,
            course_id,
            student_user_id,
            Some(REWARD_STATUS_TEACHER_REJECTED),
        )
        .await?,
        completed_count: count_student_reward_candidates(
            conn,
            course_id,
            student_user_id,
            Some(REWARD_STATUS_COMPLETED),
        )
        .await?,
        failed_count: count_student_reward_candidates(
            conn,
            course_id,
            student_user_id,
            Some(REWARD_STATUS_FAILED),
        )
        .await?,
        latest_candidate: load_latest_reward_candidate(conn, course_id, student_user_id).await?,
    })
}

async fn load_latest_reward_candidate(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    student_user_id: i32,
) -> Result<Option<TeacherStudentRewardCandidateSummaryOutput>, TeacherCourseDashboardError> {
    reward_candidates::table
        .filter(reward_candidates::course_id.eq(course_id))
        .filter(reward_candidates::student_user_id.eq(student_user_id))
        .order(reward_candidates::updated_at.desc())
        .then_order_by(reward_candidates::id.desc())
        .select((
            reward_candidates::id,
            reward_candidates::event_type,
            reward_candidates::status,
            reward_candidates::evidence,
            reward_candidates::teacher_decision_reason,
            reward_candidates::created_at,
            reward_candidates::updated_at,
        ))
        .first::<(
            i64,
            String,
            String,
            Value,
            Option<String>,
            DateTime<Utc>,
            DateTime<Utc>,
        )>(conn)
        .await
        .optional()
        .map(|row| {
            row.map(
                |(
                    id,
                    event_type,
                    status,
                    evidence,
                    teacher_decision_reason,
                    created_at,
                    updated_at,
                )| TeacherStudentRewardCandidateSummaryOutput {
                    id,
                    event_type,
                    status,
                    evidence,
                    teacher_decision_reason,
                    created_at,
                    updated_at,
                },
            )
        })
        .map_err(map_dashboard_error)
}

async fn count_student_reward_candidates(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    student_user_id: i32,
    status: Option<&str>,
) -> Result<i64, TeacherCourseDashboardError> {
    let mut query = reward_candidates::table.into_boxed();
    query = query
        .filter(reward_candidates::course_id.eq(course_id))
        .filter(reward_candidates::student_user_id.eq(student_user_id));
    if let Some(status) = status {
        query = query.filter(reward_candidates::status.eq(status));
    }

    query
        .count()
        .get_result::<i64>(conn)
        .await
        .map_err(map_dashboard_error)
}

fn map_dashboard_error(error: diesel::result::Error) -> TeacherCourseDashboardError {
    match error {
        diesel::result::Error::NotFound => TeacherCourseDashboardError::NotFound,
        other => TeacherCourseDashboardError::Database(other.to_string()),
    }
}
