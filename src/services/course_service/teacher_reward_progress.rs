use crate::db::schema::reward_candidates;
use crate::domain::rewards::candidate::status::{
    REWARD_STATUS_COMPLETED, REWARD_STATUS_FAILED, REWARD_STATUS_PENDING_TEACHER_APPROVAL,
    REWARD_STATUS_TEACHER_APPROVED, REWARD_STATUS_TEACHER_REJECTED,
};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde_json::Value;

use super::errors::TeacherCourseDashboardError;
use super::teacher_course_types::{
    TeacherStudentRewardCandidateSummary, TeacherStudentRewardProgressSummary,
};
use super::teacher_enrollment_types::TeacherCourseRewardQueueSummary;

pub(super) async fn load_teacher_student_reward_progress(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    student_user_id: i32,
) -> Result<TeacherStudentRewardProgressSummary, TeacherCourseDashboardError> {
    let reward_candidate_count =
        count_student_reward_candidates(conn, course_id, student_user_id, None).await?;
    let pending_teacher_count = count_student_reward_candidates(
        conn,
        course_id,
        student_user_id,
        Some(REWARD_STATUS_PENDING_TEACHER_APPROVAL),
    )
    .await?;
    let teacher_approved_count = count_student_reward_candidates(
        conn,
        course_id,
        student_user_id,
        Some(REWARD_STATUS_TEACHER_APPROVED),
    )
    .await?;
    let teacher_rejected_count = count_student_reward_candidates(
        conn,
        course_id,
        student_user_id,
        Some(REWARD_STATUS_TEACHER_REJECTED),
    )
    .await?;
    let completed_count = count_student_reward_candidates(
        conn,
        course_id,
        student_user_id,
        Some(REWARD_STATUS_COMPLETED),
    )
    .await?;
    let failed_count = count_student_reward_candidates(
        conn,
        course_id,
        student_user_id,
        Some(REWARD_STATUS_FAILED),
    )
    .await?;
    let latest_candidate = reward_candidates::table
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
        .optional()?
        .map(
            |(
                id,
                event_type,
                status,
                evidence,
                teacher_decision_reason,
                created_at,
                updated_at,
            )| {
                TeacherStudentRewardCandidateSummary {
                    id,
                    event_type,
                    status,
                    evidence,
                    teacher_decision_reason,
                    created_at,
                    updated_at,
                }
            },
        );

    Ok(TeacherStudentRewardProgressSummary {
        reward_candidate_count,
        pending_teacher_count,
        teacher_approved_count,
        teacher_rejected_count,
        completed_count,
        failed_count,
        latest_candidate,
    })
}

pub(super) async fn count_student_reward_candidates(
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
        .map_err(TeacherCourseDashboardError::from)
}

pub(super) async fn load_teacher_course_reward_queue_summary(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<TeacherCourseRewardQueueSummary, TeacherCourseDashboardError> {
    Ok(TeacherCourseRewardQueueSummary {
        pending_teacher_count: count_reward_candidates_by_status(
            conn,
            course_id,
            REWARD_STATUS_PENDING_TEACHER_APPROVAL,
        )
        .await?,
        teacher_approved_count: count_reward_candidates_by_status(
            conn,
            course_id,
            REWARD_STATUS_TEACHER_APPROVED,
        )
        .await?,
        failed_count: count_reward_candidates_by_status(conn, course_id, REWARD_STATUS_FAILED)
            .await?,
    })
}

async fn count_reward_candidates_by_status(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    status: &str,
) -> Result<i64, TeacherCourseDashboardError> {
    reward_candidates::table
        .filter(reward_candidates::course_id.eq(course_id))
        .filter(reward_candidates::status.eq(status))
        .count()
        .get_result::<i64>(conn)
        .await
        .map_err(TeacherCourseDashboardError::from)
}
