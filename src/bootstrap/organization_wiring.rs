use std::sync::Arc;

use actix_web::web;

use crate::application::organizations::assign_organization_member_role::OrganizationMemberRoleAssignmentUseCase;
use crate::application::organizations::get_organization_dashboard::OrganizationDashboardUseCase;
use crate::application::organizations::invite_organization_member::OrganizationMemberInviteUseCase;
use crate::application::organizations::list_organization_courses::OrganizationCourseListUseCase;
use crate::application::organizations::list_organization_member_audit::OrganizationMemberAuditUseCase;
use crate::application::organizations::list_organization_members::OrganizationMemberListUseCase;
use crate::application::organizations::list_organization_teacher_applications::OrganizationTeacherApplicationListUseCase;
use crate::application::organizations::manage_organizations::OrganizationManagementUseCase;
use crate::application::organizations::remove_organization_member::OrganizationMemberRemovalUseCase;
use crate::infra::postgres::organizations::organization_course_list_use_case::PostgresOrganizationCourseListUseCase;
use crate::infra::postgres::organizations::organization_dashboard_use_case::PostgresOrganizationDashboardUseCase;
use crate::infra::postgres::organizations::organization_management_use_case::PostgresOrganizationManagementUseCase;
use crate::infra::postgres::organizations::organization_member_audit_use_case::PostgresOrganizationMemberAuditUseCase;
use crate::infra::postgres::organizations::organization_member_invite_use_case::PostgresOrganizationMemberInviteUseCase;
use crate::infra::postgres::organizations::organization_member_list_use_case::PostgresOrganizationMemberListUseCase;
use crate::infra::postgres::organizations::organization_member_removal_use_case::PostgresOrganizationMemberRemovalUseCase;
use crate::infra::postgres::organizations::organization_member_role_assignment_use_case::PostgresOrganizationMemberRoleAssignmentUseCase;
use crate::infra::postgres::organizations::organization_teacher_application_use_case::PostgresOrganizationTeacherApplicationUseCase;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct OrganizationUseCases {
    pub course_list: Arc<dyn OrganizationCourseListUseCase>,
    pub dashboard: Arc<dyn OrganizationDashboardUseCase>,
    pub management: Arc<dyn OrganizationManagementUseCase>,
    pub member_audit: Arc<dyn OrganizationMemberAuditUseCase>,
    pub member_invite: Arc<dyn OrganizationMemberInviteUseCase>,
    pub member_list: Arc<dyn OrganizationMemberListUseCase>,
    pub member_removal: Arc<dyn OrganizationMemberRemovalUseCase>,
    pub member_role_assignment: Arc<dyn OrganizationMemberRoleAssignmentUseCase>,
    pub teacher_applications: Arc<dyn OrganizationTeacherApplicationListUseCase>,
}

pub fn build_organization_use_cases(pool: &DbPool) -> OrganizationUseCases {
    OrganizationUseCases {
        course_list: Arc::new(PostgresOrganizationCourseListUseCase::new(pool.clone())),
        dashboard: Arc::new(PostgresOrganizationDashboardUseCase::new(pool.clone())),
        management: Arc::new(PostgresOrganizationManagementUseCase::new(pool.clone())),
        member_audit: Arc::new(PostgresOrganizationMemberAuditUseCase::new(pool.clone())),
        member_invite: Arc::new(PostgresOrganizationMemberInviteUseCase::new(pool.clone())),
        member_list: Arc::new(PostgresOrganizationMemberListUseCase::new(pool.clone())),
        member_removal: Arc::new(PostgresOrganizationMemberRemovalUseCase::new(pool.clone())),
        member_role_assignment: Arc::new(PostgresOrganizationMemberRoleAssignmentUseCase::new(
            pool.clone(),
        )),
        teacher_applications: Arc::new(PostgresOrganizationTeacherApplicationUseCase::new(
            pool.clone(),
        )),
    }
}

pub fn configure_organization_app_data(
    cfg: &mut web::ServiceConfig,
    use_cases: &OrganizationUseCases,
) {
    cfg.app_data(web::Data::new(use_cases.course_list.clone()))
        .app_data(web::Data::new(use_cases.dashboard.clone()))
        .app_data(web::Data::new(use_cases.management.clone()))
        .app_data(web::Data::new(use_cases.member_audit.clone()))
        .app_data(web::Data::new(use_cases.member_invite.clone()))
        .app_data(web::Data::new(use_cases.member_list.clone()))
        .app_data(web::Data::new(use_cases.member_removal.clone()))
        .app_data(web::Data::new(use_cases.member_role_assignment.clone()))
        .app_data(web::Data::new(use_cases.teacher_applications.clone()));
}
