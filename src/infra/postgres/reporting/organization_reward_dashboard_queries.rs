use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::organization_reward_dashboard::{
    teacher_application_summary_from_statuses, OrganizationCourseRewardDashboardFact,
    OrganizationCourseRewardDashboardRowOutput, OrganizationRewardDashboardError,
    OrganizationWalletBalanceFact, OrganizationWalletBalanceRowOutput,
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
        rows.push(course_reward_row(conn, course_id, course_title, from, to).await?);
    }
    Ok(rows)
}

async fn course_reward_row(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    course_title: String,
    from: Option<NaiveDate>,
    to: Option<NaiveDate>,
) -> Result<OrganizationCourseRewardDashboardFact, OrganizationRewardDashboardError> {
    let mut query = reward_candidates::table
        .filter(reward_candidates::course_id.eq(course_id))
        .into_boxed();
    if let Some(from_date) = from {
        query = query.filter(reward_candidates::created_at.ge(start_of_day(from_date)));
    }
    if let Some(to_date) = to {
        query = query.filter(reward_candidates::created_at.le(end_of_day(to_date)));
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

    Ok(OrganizationCourseRewardDashboardFact {
        row: OrganizationCourseRewardDashboardRowOutput {
            course_id,
            course_title,
            reward_candidate_count: amounts.len() as i64,
            approved_reward_count: approved_amounts.len() as i64,
            approved_amount_total: approved_amount_total.to_string(),
        },
        approved_amount_total,
    })
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
                .map(OrganizationWalletBalanceFact::from)
                .collect()
        })
        .map_err(map_diesel_error)
}

fn start_of_day(date: NaiveDate) -> NaiveDateTime {
    NaiveDateTime::new(date, NaiveTime::from_hms_opt(0, 0, 0).unwrap())
}

fn end_of_day(date: NaiveDate) -> NaiveDateTime {
    NaiveDateTime::new(date, NaiveTime::from_hms_opt(23, 59, 59).unwrap())
}

impl From<(i32, BigDecimal)> for OrganizationWalletBalanceFact {
    fn from((wallet_id, balance): (i32, BigDecimal)) -> Self {
        Self {
            row: OrganizationWalletBalanceRowOutput {
                wallet_id,
                balance: balance.to_string(),
            },
            balance,
        }
    }
}

pub(super) fn map_diesel_error(error: diesel::result::Error) -> OrganizationRewardDashboardError {
    match error {
        diesel::result::Error::NotFound => OrganizationRewardDashboardError::NotFound,
        other => OrganizationRewardDashboardError::Database(other.to_string()),
    }
}
