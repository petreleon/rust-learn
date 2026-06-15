use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde_json::Value;

use crate::application::learning::get_teacher_course_students::{
    record_teacher_student_reward_progress_status, TeacherStudentRewardCandidateSummaryOutput,
    TeacherStudentRewardProgressSummaryOutput,
};
use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardError;
use crate::domain::rewards::candidate::event_type::RewardEventType;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::infra::postgres::schema::reward_candidates;

type LatestRewardCandidateRow = (
    i64,
    String,
    String,
    Value,
    Option<String>,
    DateTime<Utc>,
    DateTime<Utc>,
);

pub async fn load_teacher_student_reward_progress(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    student_user_id: i32,
) -> Result<TeacherStudentRewardProgressSummaryOutput, TeacherCourseDashboardError> {
    let status_counts =
        load_student_reward_candidate_status_counts(conn, course_id, student_user_id).await?;
    let mut summary = TeacherStudentRewardProgressSummaryOutput {
        reward_candidate_count: status_counts.iter().map(|(_, count)| *count).sum::<i64>(),
        pending_teacher_count: 0,
        teacher_approved_count: 0,
        teacher_rejected_count: 0,
        completed_count: 0,
        failed_count: 0,
        latest_candidate: load_latest_reward_candidate(conn, course_id, student_user_id).await?,
    };
    for (status, count) in status_counts {
        if let Ok(status) = RewardCandidateStatus::parse(&status) {
            record_teacher_student_reward_progress_status(&mut summary, status, count);
        }
    }
    Ok(summary)
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
        .first::<LatestRewardCandidateRow>(conn)
        .await
        .optional()
        .map_err(map_dashboard_error)?
        .map(latest_candidate_output)
        .transpose()
}

fn latest_candidate_output(
    (
        id,
        event_type,
        status,
        evidence,
        teacher_decision_reason,
        created_at,
        updated_at,
    ): LatestRewardCandidateRow,
) -> Result<TeacherStudentRewardCandidateSummaryOutput, TeacherCourseDashboardError> {
    let event_type = RewardEventType::parse(&event_type)
        .map_err(|error| TeacherCourseDashboardError::Database(error.to_string()))?;

    Ok(TeacherStudentRewardCandidateSummaryOutput {
        id,
        event_type,
        status,
        evidence,
        teacher_decision_reason,
        created_at,
        updated_at,
    })
}

async fn load_student_reward_candidate_status_counts(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    student_user_id: i32,
) -> Result<Vec<(String, i64)>, TeacherCourseDashboardError> {
    reward_candidates::table
        .filter(reward_candidates::course_id.eq(course_id))
        .filter(reward_candidates::student_user_id.eq(student_user_id))
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
