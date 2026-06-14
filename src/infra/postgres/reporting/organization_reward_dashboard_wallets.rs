use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::organization_reward_dashboard::{
    OrganizationRewardDashboardError, OrganizationWalletBalanceFact,
};
use crate::db::schema::wallets;
use crate::infra::postgres::reporting::organization_reward_dashboard_mappers::map_diesel_error;

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
