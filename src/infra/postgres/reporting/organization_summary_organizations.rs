use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::organization_summary::OrganizationSummaryError;
use crate::db::schema::organizations;
use crate::infra::postgres::reporting::organization_summary_mappers::map_diesel_error;

pub(super) async fn organization_name(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<String, OrganizationSummaryError> {
    organizations::table
        .find(organization_id)
        .select(organizations::name)
        .first::<String>(conn)
        .await
        .map_err(map_diesel_error)
}
