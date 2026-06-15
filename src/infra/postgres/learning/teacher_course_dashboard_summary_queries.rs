use std::collections::BTreeSet;

use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::learning::teacher_course_dashboard::{
    TeacherCourseContentSummaryOutput, TeacherCourseDashboardError,
    TeacherCourseDashboardItemOutput, TeacherCourseDashboardOrganizationOutput,
    TeacherCoursePermissionSummaryOutput, TeacherCourseRewardSummaryOutput,
};
use crate::db::schema::{
    chapters, contents, courses_organizations, organizations, reward_policies,
};
use crate::infra::postgres::learning::teacher_course_dashboard_metric_queries;
use crate::infra::postgres::models::course::Course;

pub async fn build_teacher_course_dashboard_item(
    conn: &mut AsyncPgConnection,
    course: Course,
    permissions: TeacherCoursePermissionSummaryOutput,
) -> Result<TeacherCourseDashboardItemOutput, TeacherCourseDashboardError> {
    Ok(TeacherCourseDashboardItemOutput {
        organizations: load_course_organizations(conn, course.id).await?,
        content: load_course_content_summary(conn, course.id).await?,
        rewards: load_course_reward_summary(conn, course.id).await?,
        roster: teacher_course_dashboard_metric_queries::load_course_roster_summary(
            conn, course.id,
        )
        .await?,
        reward_queue: teacher_course_dashboard_metric_queries::load_course_reward_queue_summary(
            conn, course.id,
        )
        .await?,
        permissions,
        id: course.id,
        title: course.title,
        lifecycle_status: course.lifecycle_status,
    })
}

async fn load_course_organizations(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<Vec<TeacherCourseDashboardOrganizationOutput>, TeacherCourseDashboardError> {
    let rows = courses_organizations::table
        .inner_join(
            organizations::table.on(courses_organizations::organization_id.eq(organizations::id)),
        )
        .filter(courses_organizations::course_id.eq(course_id))
        .order(courses_organizations::order.asc())
        .select((organizations::id, organizations::name))
        .load::<(i32, String)>(conn)
        .await
        .map_err(map_dashboard_error)?;

    Ok(rows
        .into_iter()
        .map(|(id, name)| TeacherCourseDashboardOrganizationOutput { id, name })
        .collect())
}

async fn load_course_content_summary(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<TeacherCourseContentSummaryOutput, TeacherCourseDashboardError> {
    let chapter_count = chapters::table
        .filter(chapters::course_id.eq(course_id))
        .count()
        .get_result::<i64>(conn)
        .await
        .map_err(map_dashboard_error)?;
    let content_types = contents::table
        .inner_join(chapters::table.on(contents::chapter_id.eq(chapters::id)))
        .filter(chapters::course_id.eq(course_id))
        .order(contents::content_type.asc())
        .select(contents::content_type)
        .load::<String>(conn)
        .await
        .map_err(map_dashboard_error)?;
    let unique_types = content_types.iter().cloned().collect::<BTreeSet<_>>();

    Ok(TeacherCourseContentSummaryOutput {
        chapter_count: chapter_count as usize,
        content_count: content_types.len(),
        content_types: unique_types.into_iter().collect(),
        has_content: !content_types.is_empty(),
    })
}

async fn load_course_reward_summary(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<TeacherCourseRewardSummaryOutput, TeacherCourseDashboardError> {
    let rows = reward_policies::table
        .filter(reward_policies::course_id.eq(course_id))
        .filter(reward_policies::active.eq(true))
        .order(reward_policies::event_type.asc())
        .select((
            reward_policies::event_type,
            reward_policies::token_amount,
            reward_policies::payment_strategy,
        ))
        .load::<(String, BigDecimal, String)>(conn)
        .await
        .map_err(map_dashboard_error)?;
    let mut event_types = BTreeSet::new();
    let mut token_amounts = BTreeSet::new();
    let mut payment_strategies = BTreeSet::new();
    for (event_type, token_amount, payment_strategy) in &rows {
        event_types.insert(event_type.clone());
        token_amounts.insert(token_amount.to_string());
        payment_strategies.insert(payment_strategy.clone());
    }

    Ok(TeacherCourseRewardSummaryOutput {
        available: !rows.is_empty(),
        active_policy_count: rows.len(),
        event_types: event_types.into_iter().collect(),
        token_amounts: token_amounts.into_iter().collect(),
        payment_strategies: payment_strategies.into_iter().collect(),
    })
}

fn map_dashboard_error(error: diesel::result::Error) -> TeacherCourseDashboardError {
    match error {
        diesel::result::Error::NotFound => TeacherCourseDashboardError::NotFound,
        other => TeacherCourseDashboardError::Database(other.to_string()),
    }
}
