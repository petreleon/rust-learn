use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::organization_reward_dashboard::{
    teacher_application_summary_from_statuses, OrganizationRewardDashboardError,
    TeacherApplicationDashboardSummaryOutput,
};
use crate::db::schema::teacher_applications;
use crate::infra::postgres::reporting::organization_reward_dashboard_mappers::map_diesel_error;

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
