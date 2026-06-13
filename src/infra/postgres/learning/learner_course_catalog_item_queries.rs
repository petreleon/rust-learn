use std::collections::BTreeSet;

use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::learning::learner_course_catalog::{
    LearnerCourseCatalogError, LearnerCourseCatalogItemOutput,
    LearnerCourseCatalogOrganizationOutput, LearnerCourseCatalogTeacherOutput,
    LearnerCourseContentSummaryOutput, LearnerCourseRewardSummaryOutput,
};
use crate::db::schema::{
    chapters, contents, course_roles, courses_organizations, organizations, reward_policies,
    user_role_course, users,
};
use crate::infra::postgres::learning::{
    learner_course_access_queries, learner_course_enrollment_summary_queries,
};
use crate::models::course::Course;

pub async fn build_learner_course_catalog_item(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course: Course,
) -> Result<LearnerCourseCatalogItemOutput, LearnerCourseCatalogError> {
    let organizations = load_learner_course_organizations(conn, course.id).await?;
    let teachers = load_learner_course_teachers(conn, course.id).await?;
    let content = load_learner_course_content_summary(conn, course.id).await?;
    let rewards = load_learner_course_reward_summary(conn, course.id).await?;
    let access =
        learner_course_access_queries::build_learner_course_access(conn, actor_user_id, course.id)
            .await?;
    let enrollment = learner_course_enrollment_summary_queries::build_learner_course_enrollment(
        conn,
        actor_user_id,
        course.id,
        &course.lifecycle_status,
        access.can_request_join,
    )
    .await?;

    Ok(LearnerCourseCatalogItemOutput {
        id: course.id,
        title: course.title,
        lifecycle_status: course.lifecycle_status,
        description: course.description.clone(),
        topics: parse_topics(course.topics.as_deref()),
        prerequisites: course.prerequisites.map(|p| vec![p]).unwrap_or_default(),
        organizations,
        teachers,
        content,
        rewards,
        enrollment,
        access,
    })
}

async fn load_learner_course_organizations(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<Vec<LearnerCourseCatalogOrganizationOutput>, LearnerCourseCatalogError> {
    let rows = courses_organizations::table
        .inner_join(
            organizations::table.on(courses_organizations::organization_id.eq(organizations::id)),
        )
        .filter(courses_organizations::course_id.eq(course_id))
        .order(courses_organizations::order.asc())
        .select((organizations::id, organizations::name))
        .load::<(i32, String)>(conn)
        .await
        .map_err(map_learning_error)?;

    Ok(rows
        .into_iter()
        .map(|(id, name)| LearnerCourseCatalogOrganizationOutput { id, name })
        .collect())
}

async fn load_learner_course_teachers(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<Vec<LearnerCourseCatalogTeacherOutput>, LearnerCourseCatalogError> {
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
        .map_err(map_learning_error)?;

    Ok(rows
        .into_iter()
        .map(|(id, name)| LearnerCourseCatalogTeacherOutput { id, name })
        .collect())
}

async fn load_learner_course_content_summary(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<LearnerCourseContentSummaryOutput, LearnerCourseCatalogError> {
    let chapter_count = chapters::table
        .filter(chapters::course_id.eq(course_id))
        .count()
        .get_result::<i64>(conn)
        .await
        .map_err(map_learning_error)?;
    let content_types = contents::table
        .inner_join(chapters::table.on(contents::chapter_id.eq(chapters::id)))
        .filter(chapters::course_id.eq(course_id))
        .order(contents::content_type.asc())
        .select(contents::content_type)
        .load::<String>(conn)
        .await
        .map_err(map_learning_error)?;

    let unique_types = content_types.iter().cloned().collect::<BTreeSet<_>>();
    Ok(LearnerCourseContentSummaryOutput {
        chapter_count: chapter_count as usize,
        content_count: content_types.len(),
        content_types: unique_types.into_iter().collect(),
        has_content: !content_types.is_empty(),
    })
}

async fn load_learner_course_reward_summary(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<LearnerCourseRewardSummaryOutput, LearnerCourseCatalogError> {
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
        .map_err(map_learning_error)?;

    let mut event_types = BTreeSet::new();
    let mut token_amounts = BTreeSet::new();
    let mut payment_strategies = BTreeSet::new();
    for (event_type, token_amount, payment_strategy) in &rows {
        event_types.insert(event_type.clone());
        token_amounts.insert(token_amount.to_string());
        payment_strategies.insert(payment_strategy.clone());
    }

    Ok(LearnerCourseRewardSummaryOutput {
        available: !rows.is_empty(),
        active_policy_count: rows.len(),
        event_types: event_types.into_iter().collect(),
        token_amounts: token_amounts.into_iter().collect(),
        payment_strategies: payment_strategies.into_iter().collect(),
    })
}

fn parse_topics(topics: Option<&str>) -> Vec<String> {
    topics
        .and_then(|value| serde_json::from_str::<Vec<String>>(value).ok())
        .unwrap_or_default()
}

fn map_learning_error(error: diesel::result::Error) -> LearnerCourseCatalogError {
    match error {
        diesel::result::Error::NotFound => LearnerCourseCatalogError::NotFound,
        other => LearnerCourseCatalogError::Database(other.to_string()),
    }
}
