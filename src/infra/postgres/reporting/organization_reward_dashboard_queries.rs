use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::organization_reward_dashboard::{
    organization_reward_dashboard_date_window, teacher_application_summary_from_statuses,
    OrganizationCourseRewardDashboardFact, OrganizationRewardDashboardDateWindow,
    OrganizationRewardDashboardError, OrganizationWalletBalanceFact,
    TeacherApplicationDashboardSummaryOutput,
};
use crate::db::schema::{
    courses, courses_organizations, reward_candidates, teacher_applications, wallets,
};

pub(super) async fn sponsored_teacher_application_summary(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<TeacherApplicationDashboardSummaryOutput, OrganizationRewardDashboardError> {
    let statuses = teacher_applications::table
        .filter(
            teacher_applications::organization_sponsor_id
                .eq(Some(organization_id))
                .or(teacher_applications::requested_organization_id.eq(Some(organization_id))),
        )
        .select(teacher_applications::status)
        .load::<String>(conn)
        .await
        .map_err(map_diesel_error)?;

    Ok(teacher_application_summary_from_statuses(statuses))
}

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
    let approved_amounts = amounts.iter().flatten().cloned().collect::<Vec<_>>();
    let approved_amount_total = approved_amounts
        .iter()
        .cloned()
        .fold(BigDecimal::from(0), |sum, amount| sum + amount);

    Ok(OrganizationCourseRewardDashboardFact::new(
        course_id,
        course_title,
        amounts.len() as i64,
        approved_amounts.len() as i64,
        approved_amount_total,
    ))
}

pub(super) async fn wallet_balance_rows(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<Vec<OrganizationWalletBalanceFact>, OrganizationRewardDashboardError> {
    wallets::table
        .filter(wallets::organization_id.eq(Some(organization_id)))
        .filter(wallets::user_id.is_null())
        .select((wallets::id, wallets::value))
        .order(wallets::id.asc())
        .load::<(i32, BigDecimal)>(conn)
        .await
        .map(|rows| {
            rows.into_iter()
                .map(|(wallet_id, balance)| OrganizationWalletBalanceFact::new(wallet_id, balance))
                .collect()
        })
        .map_err(map_diesel_error)
}

pub(super) fn map_diesel_error(error: diesel::result::Error) -> OrganizationRewardDashboardError {
    match error {
        diesel::result::Error::NotFound => OrganizationRewardDashboardError::NotFound,
        other => OrganizationRewardDashboardError::Database(other.to_string()),
    }
}
