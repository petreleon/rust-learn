use crate::db::schema::{
    courses, courses_organizations, delegated_permissions, teacher_applications,
};
use crate::domain::learning::course::status::{
    COURSE_STATUS_APPROVED, COURSE_STATUS_ARCHIVED, COURSE_STATUS_DRAFT,
    COURSE_STATUS_NEEDS_CHANGES, COURSE_STATUS_PUBLISHED, COURSE_STATUS_SUBMITTED,
    COURSE_STATUS_SUSPENDED,
};
use crate::domain::teacher_applications::status::{
    TEACHER_APPLICATION_STATUS_APPROVED, TEACHER_APPLICATION_STATUS_NEEDS_CHANGES,
    TEACHER_APPLICATION_STATUS_REJECTED, TEACHER_APPLICATION_STATUS_SUBMITTED,
};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use super::dashboard_types_and_reads::OrganizationDashboardError;
use super::organization_dashboard_alerts::build_organization_member_builders;
use super::support::{
    OrganizationDashboardCourseSummary, OrganizationDashboardMemberSummary,
    OrganizationDashboardTeacherApplicationSummary,
};

pub(super) async fn log_organization_member_event(
    conn: &mut diesel_async::AsyncPgConnection,
    organization_id: i32,
    actor_user_id: Option<i32>,
    target_user_id: i32,
    event_type: &str,
    role_name: Option<&str>,
    reason: Option<&str>,
) -> QueryResult<()> {
    use crate::db::schema::organization_member_audit_events;
    use crate::models::organization_member_audit_event::NewOrganizationMemberAuditEvent;

    diesel::insert_into(organization_member_audit_events::table)
        .values(NewOrganizationMemberAuditEvent {
            organization_id,
            actor_user_id,
            target_user_id,
            event_type: event_type.to_string(),
            role_name: role_name.map(|s| s.to_string()),
            reason: reason.map(|s| s.to_string()),
        })
        .execute(conn)
        .await?;
    Ok(())
}

pub(super) async fn organization_dashboard_member_summary(
    conn: &mut diesel_async::AsyncPgConnection,
    organization_id: i32,
) -> Result<OrganizationDashboardMemberSummary, OrganizationDashboardError> {
    let members = build_organization_member_builders(conn, organization_id).await?;
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
        .await?;

    Ok(OrganizationDashboardMemberSummary {
        available: true,
        missing_permissions: vec![],
        total,
        verified_email_count,
        kyc_ready_count,
        delegated_permission_count,
    })
}

pub(super) async fn organization_dashboard_course_summary(
    conn: &mut diesel_async::AsyncPgConnection,
    organization_id: i32,
) -> Result<OrganizationDashboardCourseSummary, OrganizationDashboardError> {
    let statuses = courses_organizations::table
        .inner_join(courses::table.on(courses_organizations::course_id.eq(courses::id)))
        .filter(courses_organizations::organization_id.eq(organization_id))
        .select(courses::lifecycle_status)
        .load::<String>(conn)
        .await?;

    let mut summary = OrganizationDashboardCourseSummary {
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

pub(super) async fn organization_dashboard_teacher_application_summary(
    conn: &mut diesel_async::AsyncPgConnection,
    organization_id: i32,
) -> Result<OrganizationDashboardTeacherApplicationSummary, OrganizationDashboardError> {
    let statuses = teacher_applications::table
        .filter(
            teacher_applications::organization_sponsor_id
                .eq(Some(organization_id))
                .or(teacher_applications::requested_organization_id.eq(Some(organization_id))),
        )
        .select(teacher_applications::status)
        .load::<String>(conn)
        .await?;

    let mut summary = OrganizationDashboardTeacherApplicationSummary {
        available: true,
        missing_permissions: vec![],
        ..Default::default()
    };
    summary.total = statuses.len() as i64;
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
