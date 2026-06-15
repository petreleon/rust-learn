use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::organization_reward_dashboard::{
    organization_course_reward_fact_from_amounts, organization_reward_dashboard_date_window,
    OrganizationCourseRewardDashboardFact, OrganizationRewardDashboardDateWindow,
    OrganizationRewardDashboardError,
};
use crate::infra::postgres::reporting::organization_reward_dashboard_mappers::map_diesel_error;
use crate::infra::postgres::schema::{courses, courses_organizations, reward_candidates};

pub(super) async fn course_reward_rows(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
    from: Option<NaiveDate>,
    to: Option<NaiveDate>,
) -> Result<Vec<OrganizationCourseRewardDashboardFact>, OrganizationRewardDashboardError> {
    let window = organization_reward_dashboard_date_window(from, to);
    let courses = courses_organizations::table
        .inner_join(courses::table.on(courses_organizations::course_id.eq(courses::id)))
        .filter(courses_organizations::organization_id.eq(organization_id))
        .select((courses::id, courses::title))
        .order(courses::title.asc())
        .load::<(i32, String)>(conn)
        .await
        .map_err(map_diesel_error)?;

    let mut rows = Vec::new();
    for (course_id, course_title) in courses {
        rows.push(course_reward_row(conn, course_id, course_title, window).await?);
    }
    Ok(rows)
}

async fn course_reward_row(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    course_title: String,
    window: OrganizationRewardDashboardDateWindow,
) -> Result<OrganizationCourseRewardDashboardFact, OrganizationRewardDashboardError> {
    let mut query = reward_candidates::table
        .filter(reward_candidates::course_id.eq(course_id))
        .into_boxed();
    if let Some(from_date) = window.starts_at() {
        query = query.filter(reward_candidates::created_at.ge(from_date));
    }
    if let Some(to_date) = window.ends_at() {
        query = query.filter(reward_candidates::created_at.le(to_date));
    }

    let amounts = query
        .select(reward_candidates::approved_amount)
        .load::<Option<BigDecimal>>(conn)
        .await
        .map_err(map_diesel_error)?;

    Ok(organization_course_reward_fact_from_amounts(
        course_id,
        course_title,
        amounts,
    ))
}
