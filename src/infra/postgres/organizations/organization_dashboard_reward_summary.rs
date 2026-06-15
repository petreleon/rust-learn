use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::organizations::get_organization_dashboard::{
    record_organization_dashboard_reward_status, OrganizationDashboardError,
    OrganizationDashboardRewardSummaryOutput, OrganizationDashboardWalletSummaryOutput,
};
use crate::application::reporting::organization_reward_dashboard::load_organization_reward_dashboard;
use crate::db::schema::{courses_organizations, reward_candidates, wallets};
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::infra::postgres::organizations::organization_dashboard_mappers::{
    map_dashboard_error, map_reward_dashboard_error,
};
use crate::infra::postgres::reporting::organization_reward_dashboard_store::PostgresOrganizationRewardDashboardStore;

pub async fn load_reward_summary(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<OrganizationDashboardRewardSummaryOutput, OrganizationDashboardError> {
    let reward_dashboard = {
        let mut store = PostgresOrganizationRewardDashboardStore::new(conn);
        load_organization_reward_dashboard(&mut store, organization_id, None, None)
            .await
            .map_err(map_reward_dashboard_error)?
    };
    let course_ids = load_organization_course_ids(conn, organization_id).await?;
    let statuses = load_reward_candidate_statuses(conn, &course_ids).await?;

    let mut summary = OrganizationDashboardRewardSummaryOutput {
        available: true,
        missing_permissions: vec![],
        reward_candidate_count: reward_dashboard.course_reward_count,
        approved_reward_count: reward_dashboard.approved_reward_count,
        approved_amount_total: reward_dashboard.approved_amount_total,
        failed_count: 0,
        needs_reconciliation_count: 0,
    };
    for status in statuses {
        if let Ok(status) = RewardCandidateStatus::parse(&status) {
            record_organization_dashboard_reward_status(&mut summary, status);
        }
    }

    Ok(summary)
}

pub async fn load_wallet_summary(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<OrganizationDashboardWalletSummaryOutput, OrganizationDashboardError> {
    let balances = wallets::table
        .filter(wallets::organization_id.eq(Some(organization_id)))
        .filter(wallets::user_id.is_null())
        .select(wallets::value)
        .load::<BigDecimal>(conn)
        .await
        .map_err(map_dashboard_error)?;
    let balance_total = balances
        .iter()
        .cloned()
        .fold(BigDecimal::from(0), |total, balance| total + balance);

    Ok(OrganizationDashboardWalletSummaryOutput {
        available: true,
        missing_permissions: vec![],
        wallet_count: balances.len() as i64,
        balance_total: balance_total.to_string(),
    })
}

async fn load_organization_course_ids(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<Vec<i32>, OrganizationDashboardError> {
    courses_organizations::table
        .filter(courses_organizations::organization_id.eq(organization_id))
        .select(courses_organizations::course_id)
        .load::<i32>(conn)
        .await
        .map_err(map_dashboard_error)
}

async fn load_reward_candidate_statuses(
    conn: &mut AsyncPgConnection,
    course_ids: &[i32],
) -> Result<Vec<String>, OrganizationDashboardError> {
    if course_ids.is_empty() {
        return Ok(Vec::new());
    }

    reward_candidates::table
        .filter(reward_candidates::course_id.eq_any(course_ids))
        .select(reward_candidates::status)
        .load::<String>(conn)
        .await
        .map_err(map_dashboard_error)
}
