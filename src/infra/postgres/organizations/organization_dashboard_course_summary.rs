use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::organizations::get_organization_dashboard::{
    OrganizationDashboardCourseSummaryOutput, OrganizationDashboardError,
};
use crate::domain::learning::course::status::{
    COURSE_STATUS_APPROVED, COURSE_STATUS_ARCHIVED, COURSE_STATUS_DRAFT,
    COURSE_STATUS_NEEDS_CHANGES, COURSE_STATUS_PUBLISHED, COURSE_STATUS_SUBMITTED,
    COURSE_STATUS_SUSPENDED,
};
use crate::infra::postgres::organizations::organization_dashboard_mappers::map_dashboard_error;
use crate::infra::postgres::schema::{courses, courses_organizations};

pub async fn load_course_summary(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<OrganizationDashboardCourseSummaryOutput, OrganizationDashboardError> {
    let statuses = courses_organizations::table
        .inner_join(courses::table.on(courses_organizations::course_id.eq(courses::id)))
        .filter(courses_organizations::organization_id.eq(organization_id))
        .select(courses::lifecycle_status)
        .load::<String>(conn)
        .await
        .map_err(map_dashboard_error)?;

    let mut summary = OrganizationDashboardCourseSummaryOutput {
        available: true,
        missing_permissions: vec![],
        total: statuses.len() as i64,
        draft: 0,
        submitted: 0,
        needs_changes: 0,
        approved: 0,
        published: 0,
        suspended: 0,
        archived: 0,
    };

    for status in statuses {
        match status.as_str() {
            COURSE_STATUS_DRAFT => summary.draft += 1,
            COURSE_STATUS_SUBMITTED => summary.submitted += 1,
            COURSE_STATUS_NEEDS_CHANGES => summary.needs_changes += 1,
            COURSE_STATUS_APPROVED => summary.approved += 1,
            COURSE_STATUS_PUBLISHED => summary.published += 1,
            COURSE_STATUS_SUSPENDED => summary.suspended += 1,
            COURSE_STATUS_ARCHIVED => summary.archived += 1,
            _ => {}
        }
    }

    Ok(summary)
}
