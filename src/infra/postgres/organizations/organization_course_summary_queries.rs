use std::collections::BTreeSet;

use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::organizations::list_organization_courses::{
    OrganizationCourseContentSummaryOutput, OrganizationCourseListError,
    OrganizationCourseListItemOutput, OrganizationCoursePermissionSummaryOutput,
    OrganizationCourseRewardSummaryOutput, OrganizationCourseTeacherOutput,
};
use crate::db::schema::{
    chapters, contents, course_roles, reward_policies, user_role_course, users,
};
use crate::infra::postgres::organizations::organization_course_metric_queries;
use crate::models::course::Course;

pub async fn build_organization_course_list_item(
    conn: &mut AsyncPgConnection,
    course: Course,
    permissions: OrganizationCoursePermissionSummaryOutput,
) -> Result<OrganizationCourseListItemOutput, OrganizationCourseListError> {
    let teachers = load_course_teachers(conn, course.id).await?;
    let content = load_course_content_summary(conn, course.id).await?;
    let rewards = load_course_reward_summary(conn, course.id).await?;
    let roster =
        organization_course_metric_queries::load_course_roster_summary(conn, course.id).await?;
    let reward_queue =
        organization_course_metric_queries::load_course_reward_queue_summary(conn, course.id)
            .await?;

    Ok(OrganizationCourseListItemOutput {
        id: course.id,
        title: course.title,
        lifecycle_status: course.lifecycle_status,
        teachers,
        content,
        rewards,
        roster,
        reward_queue,
        permissions,
    })
}

async fn load_course_teachers(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<Vec<OrganizationCourseTeacherOutput>, OrganizationCourseListError> {
    let rows = user_role_course::table
        .inner_join(
            course_roles::table
                .on(user_role_course::course_role_id.eq(course_roles::id.nullable())),
        )
        .inner_join(users::table.on(user_role_course::user_id.eq(users::id.nullable())))
        .filter(user_role_course::course_id.eq(course_id))
        .filter(course_roles::name.eq("TEACHER"))
        .order(users::name.asc())
        .select((users::id, users::name))
        .load::<(i32, String)>(conn)
        .await
        .map_err(map_organization_error)?;

    Ok(rows
        .into_iter()
        .map(|(id, name)| OrganizationCourseTeacherOutput { id, name })
        .collect())
}

async fn load_course_content_summary(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<OrganizationCourseContentSummaryOutput, OrganizationCourseListError> {
    let chapter_count = chapters::table
        .filter(chapters::course_id.eq(course_id))
        .count()
        .get_result::<i64>(conn)
        .await
        .map_err(map_organization_error)?;
    let content_types = contents::table
        .inner_join(chapters::table.on(contents::chapter_id.eq(chapters::id)))
        .filter(chapters::course_id.eq(course_id))
        .order(contents::content_type.asc())
        .select(contents::content_type)
        .load::<String>(conn)
        .await
        .map_err(map_organization_error)?;

    let unique_types = content_types.iter().cloned().collect::<BTreeSet<_>>();
    Ok(OrganizationCourseContentSummaryOutput {
        chapter_count: chapter_count as usize,
        content_count: content_types.len(),
        content_types: unique_types.into_iter().collect(),
        has_content: !content_types.is_empty(),
    })
}

async fn load_course_reward_summary(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<OrganizationCourseRewardSummaryOutput, OrganizationCourseListError> {
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
        .map_err(map_organization_error)?;

    let mut event_types = BTreeSet::new();
    let mut token_amounts = BTreeSet::new();
    let mut payment_strategies = BTreeSet::new();
    for (event_type, token_amount, payment_strategy) in &rows {
        event_types.insert(event_type.clone());
        token_amounts.insert(token_amount.to_string());
        payment_strategies.insert(payment_strategy.clone());
    }

    Ok(OrganizationCourseRewardSummaryOutput {
        available: !rows.is_empty(),
        active_policy_count: rows.len(),
        event_types: event_types.into_iter().collect(),
        token_amounts: token_amounts.into_iter().collect(),
        payment_strategies: payment_strategies.into_iter().collect(),
    })
}

fn map_organization_error(error: diesel::result::Error) -> OrganizationCourseListError {
    match error {
        diesel::result::Error::NotFound => OrganizationCourseListError::NotFound,
        other => OrganizationCourseListError::Database(other.to_string()),
    }
}
