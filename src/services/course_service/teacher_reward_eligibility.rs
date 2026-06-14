use crate::db::schema::{courses_organizations, reward_policies};
use crate::domain::rewards::policy::{
    REWARD_POLICY_SCOPE_COURSE, REWARD_POLICY_SCOPE_ORGANIZATION, REWARD_POLICY_SCOPE_PLATFORM,
};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use std::collections::BTreeSet;

use super::errors::TeacherCourseDashboardError;
use super::teacher_course_types::{
    TeacherCourseRewardEligibilitySummary, TeacherStudentRewardEligibilitySummary,
};
use super::teacher_reward_progress::count_student_reward_candidates;

pub(super) async fn load_teacher_course_reward_eligibility_summary(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<TeacherCourseRewardEligibilitySummary, TeacherCourseDashboardError> {
    let mut policy_events = reward_policies::table
        .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_COURSE))
        .filter(reward_policies::course_id.eq(Some(course_id)))
        .filter(reward_policies::active.eq(true))
        .select(reward_policies::event_type)
        .load::<String>(conn)
        .await?;

    let organization_ids = courses_organizations::table
        .filter(courses_organizations::course_id.eq(course_id))
        .select(courses_organizations::organization_id)
        .load::<i32>(conn)
        .await?;
    if !organization_ids.is_empty() {
        policy_events.extend(
            reward_policies::table
                .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_ORGANIZATION))
                .filter(reward_policies::organization_id.eq_any(organization_ids))
                .filter(reward_policies::course_id.is_null())
                .filter(reward_policies::active.eq(true))
                .select(reward_policies::event_type)
                .load::<String>(conn)
                .await?,
        );
    }

    policy_events.extend(
        reward_policies::table
            .filter(reward_policies::scope_type.eq(REWARD_POLICY_SCOPE_PLATFORM))
            .filter(reward_policies::organization_id.is_null())
            .filter(reward_policies::course_id.is_null())
            .filter(reward_policies::active.eq(true))
            .select(reward_policies::event_type)
            .load::<String>(conn)
            .await?,
    );

    let event_types = policy_events.iter().cloned().collect::<BTreeSet<_>>();
    Ok(TeacherCourseRewardEligibilitySummary {
        active_policy_count: policy_events.len(),
        event_types: event_types.into_iter().collect(),
        supported: true,
    })
}

pub(super) async fn load_teacher_student_reward_eligibility(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    student_user_id: i32,
    course_eligibility: &TeacherCourseRewardEligibilitySummary,
) -> Result<TeacherStudentRewardEligibilitySummary, TeacherCourseDashboardError> {
    let candidate_count =
        count_student_reward_candidates(conn, course_id, student_user_id, None).await?;
    Ok(teacher_student_reward_eligibility_from_count(
        course_eligibility,
        candidate_count,
    ))
}

pub(super) fn teacher_student_reward_eligibility_from_count(
    course_eligibility: &TeacherCourseRewardEligibilitySummary,
    reward_candidate_count: i64,
) -> TeacherStudentRewardEligibilitySummary {
    TeacherStudentRewardEligibilitySummary {
        active_policy_count: course_eligibility.active_policy_count,
        event_types: course_eligibility.event_types.clone(),
        reward_candidate_count,
        supported: course_eligibility.supported,
    }
}
