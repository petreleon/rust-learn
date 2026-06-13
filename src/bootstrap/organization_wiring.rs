use std::sync::Arc;

use crate::application::organizations::get_organization_dashboard::OrganizationDashboardUseCase;
use crate::application::organizations::invite_organization_member::OrganizationMemberInviteUseCase;
use crate::application::organizations::list_organization_courses::OrganizationCourseListUseCase;
use crate::application::organizations::list_organization_member_audit::OrganizationMemberAuditUseCase;
use crate::application::organizations::list_organization_members::OrganizationMemberListUseCase;
use crate::application::organizations::remove_organization_member::OrganizationMemberRemovalUseCase;
use crate::db::DbPool;
use crate::infra::postgres::organizations::organization_course_list_use_case::PostgresOrganizationCourseListUseCase;
use crate::infra::postgres::organizations::organization_dashboard_use_case::PostgresOrganizationDashboardUseCase;
use crate::infra::postgres::organizations::organization_member_audit_use_case::PostgresOrganizationMemberAuditUseCase;
use crate::infra::postgres::organizations::organization_member_invite_use_case::PostgresOrganizationMemberInviteUseCase;
use crate::infra::postgres::organizations::organization_member_list_use_case::PostgresOrganizationMemberListUseCase;
use crate::infra::postgres::organizations::organization_member_removal_use_case::PostgresOrganizationMemberRemovalUseCase;

#[derive(Clone)]
pub struct OrganizationUseCases {
    pub course_list: Arc<dyn OrganizationCourseListUseCase>,
    pub dashboard: Arc<dyn OrganizationDashboardUseCase>,
    pub member_audit: Arc<dyn OrganizationMemberAuditUseCase>,
    pub member_invite: Arc<dyn OrganizationMemberInviteUseCase>,
    pub member_list: Arc<dyn OrganizationMemberListUseCase>,
    pub member_removal: Arc<dyn OrganizationMemberRemovalUseCase>,
}

pub fn build_organization_use_cases(pool: &DbPool) -> OrganizationUseCases {
    OrganizationUseCases {
        course_list: Arc::new(PostgresOrganizationCourseListUseCase::new(pool.clone())),
        dashboard: Arc::new(PostgresOrganizationDashboardUseCase::new(pool.clone())),
        member_audit: Arc::new(PostgresOrganizationMemberAuditUseCase::new(pool.clone())),
        member_invite: Arc::new(PostgresOrganizationMemberInviteUseCase::new(pool.clone())),
        member_list: Arc::new(PostgresOrganizationMemberListUseCase::new(pool.clone())),
        member_removal: Arc::new(PostgresOrganizationMemberRemovalUseCase::new(pool.clone())),
    }
}
