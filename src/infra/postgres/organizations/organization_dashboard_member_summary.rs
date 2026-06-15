use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::organizations::get_organization_dashboard::{
    OrganizationDashboardError, OrganizationDashboardMemberSummaryOutput,
};
use crate::infra::postgres::organizations::organization_dashboard_mappers::{
    map_dashboard_error, map_member_list_error,
};
use crate::infra::postgres::organizations::organization_member_builders;
use crate::infra::postgres::schema::delegated_permissions;

pub async fn load_member_summary(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<OrganizationDashboardMemberSummaryOutput, OrganizationDashboardError> {
    let members =
        organization_member_builders::build_organization_member_builders(conn, organization_id)
            .await
            .map_err(map_member_list_error)?;
    let total = members.len() as i64;
    let verified_email_count = members
        .values()
        .filter(|member| member.email_verified)
        .count() as i64;
    let kyc_ready_count = members
        .values()
        .filter(|member| member.kyc_verified)
        .count() as i64;

    let now: DateTime<Utc> = Utc::now();
    let delegated_permission_count = delegated_permissions::table
        .filter(delegated_permissions::scope_type.eq("organization"))
        .filter(delegated_permissions::organization_id.eq(Some(organization_id)))
        .filter(delegated_permissions::course_id.is_null())
        .filter(delegated_permissions::revoked_at.is_null())
        .filter(
            delegated_permissions::expires_at
                .is_null()
                .or(delegated_permissions::expires_at.gt(now)),
        )
        .count()
        .get_result::<i64>(conn)
        .await
        .map_err(map_dashboard_error)?;

    Ok(OrganizationDashboardMemberSummaryOutput {
        available: true,
        missing_permissions: vec![],
        total,
        verified_email_count,
        kyc_ready_count,
        delegated_permission_count,
    })
}
