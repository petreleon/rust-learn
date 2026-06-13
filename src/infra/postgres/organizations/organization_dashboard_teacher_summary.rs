use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::organizations::get_organization_dashboard::{
    OrganizationDashboardError, OrganizationDashboardTeacherApplicationSummaryOutput,
};
use crate::db::schema::teacher_applications;
use crate::infra::postgres::organizations::organization_dashboard_mappers::map_dashboard_error;
use crate::models::teacher_application::{
    TEACHER_APPLICATION_STATUS_APPROVED, TEACHER_APPLICATION_STATUS_NEEDS_CHANGES,
    TEACHER_APPLICATION_STATUS_REJECTED, TEACHER_APPLICATION_STATUS_SUBMITTED,
};

pub async fn load_teacher_application_summary(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<OrganizationDashboardTeacherApplicationSummaryOutput, OrganizationDashboardError> {
    let statuses = teacher_applications::table
        .filter(
            teacher_applications::organization_sponsor_id
                .eq(Some(organization_id))
                .or(teacher_applications::requested_organization_id.eq(Some(organization_id))),
        )
        .select(teacher_applications::status)
        .load::<String>(conn)
        .await
        .map_err(map_dashboard_error)?;

    let mut summary = OrganizationDashboardTeacherApplicationSummaryOutput {
        available: true,
        missing_permissions: vec![],
        total: statuses.len() as i64,
        submitted: 0,
        needs_changes: 0,
        approved: 0,
        rejected: 0,
    };

    for status in statuses {
        match status.as_str() {
            TEACHER_APPLICATION_STATUS_SUBMITTED => summary.submitted += 1,
            TEACHER_APPLICATION_STATUS_NEEDS_CHANGES => summary.needs_changes += 1,
            TEACHER_APPLICATION_STATUS_APPROVED => summary.approved += 1,
            TEACHER_APPLICATION_STATUS_REJECTED => summary.rejected += 1,
            _ => {}
        }
    }

    Ok(summary)
}
