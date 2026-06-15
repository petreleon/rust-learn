use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::organization_summary::OrganizationSummaryError;
use crate::db::schema::wallets;
use crate::infra::postgres::reporting::organization_summary_mappers::map_diesel_error;

pub(super) async fn organization_wallet_count(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<i64, OrganizationSummaryError> {
    wallets::table
        .filter(wallets::organization_id.eq(organization_id))
        .count()
        .get_result(conn)
        .await
        .map_err(map_diesel_error)
}
