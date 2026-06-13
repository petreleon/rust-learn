use std::collections::BTreeSet;

use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardError;
use crate::application::learning::teacher_course_enrollment::{
    TeacherCourseRewardEligibilitySummaryOutput, TeacherStudentRewardEligibilitySummaryOutput,
};
use crate::db::schema::{courses_organizations, reward_candidates, reward_policies};
use crate::domain::rewards::policy::{
    REWARD_POLICY_SCOPE_COURSE, REWARD_POLICY_SCOPE_ORGANIZATION, REWARD_POLICY_SCOPE_PLATFORM,
};

pub async fn load_teacher_course_reward_eligibility_summary(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<TeacherCourseRewardEligibilitySummaryOutput, TeacherCourseDashboardError> {
    let mut policy_events = reward_policies::table
        .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_COURSE))
        .filter(reward_policies::course_id.eq(Some(course_id)))
        .filter(reward_policies::active.eq(true))
        .select(reward_policies::event_type)
        .load::<String>(conn)
        .await
        .map_err(map_dashboard_error)?;

    let organization_ids = courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .select(courses_organizations::organization_id)
        .load::<i32>(conn)
        .await
        .map_err(map_dashboard_error)?;
    if !organization_ids.is_empty() {
        policy_events.extend(load_organization_policy_events(conn, organization_ids).await?);
    }

    policy_events.extend(load_platform_policy_events(conn).await?);
    let event_types = policy_events.iter().cloned().collect::<BTreeSet<_>>();
    Ok(TeacherCourseRewardEligibilitySummaryOutput {
        active_policy_count: policy_events.len(),
        event_types: event_types.into_iter().collect(),
        supported: true,
    })
}

pub async fn load_teacher_student_reward_eligibility(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    student_user_id: i32,
    course_eligibility: &TeacherCourseRewardEligibilitySummaryOutput,
) -> Result<TeacherStudentRewardEligibilitySummaryOutput, TeacherCourseDashboardError> {
    let count = count_student_reward_candidates(conn, course_id, student_user_id).await?;
    Ok(teacher_student_reward_eligibility_from_count(
        course_eligibility,
        count,
    ))
}

fn teacher_student_reward_eligibility_from_count(
    course_eligibility: &TeacherCourseRewardEligibilitySummaryOutput,
    reward_candidate_count: i64,
) -> TeacherStudentRewardEligibilitySummaryOutput {
    TeacherStudentRewardEligibilitySummaryOutput {
        active_policy_count: course_eligibility.active_policy_count,
        event_types: course_eligibility.event_types.clone(),
        reward_candidate_count,
        supported: course_eligibility.supported,
    }
}

async fn load_organization_policy_events(
    conn: &mut AsyncPgConnection,
    organization_ids: Vec<i32>,
) -> Result<Vec<String>, TeacherCourseDashboardError> {
    reward_policies::table
        .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_ORGANIZATION))
        .filter(reward_policies::organization_id.eq_any(organization_ids))
        .filter(reward_policies::course_id.is_null())
        .filter(reward_policies::active.eq(true))
        .select(reward_policies::event_type)
        .load::<String>(conn)
        .await
        .map_err(map_dashboard_error)
}

async fn load_platform_policy_events(
    conn: &mut AsyncPgConnection,
) -> Result<Vec<String>, TeacherCourseDashboardError> {
    reward_policies::table
        .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_PLATFORM))
        .filter(reward_policies::organization_id.is_null())
        .filter(reward_policies::course_id.is_null())
        .filter(reward_policies::active.eq(true))
        .select(reward_policies::event_type)
        .load::<String>(conn)
        .await
        .map_err(map_dashboard_error)
}

async fn count_student_reward_candidates(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    student_user_id: i32,
) -> Result<i64, TeacherCourseDashboardError> {
    reward_candidates::table
        .filter(reward_candidates::course_id.eq(course_id))
        .filter(reward_candidates::student_user_id.eq(student_user_id))
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
