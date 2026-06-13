use std::sync::Arc;

use actix_web::web;
use chrono::Utc;
use futures::future::{ready, BoxFuture, FutureExt};
use rust_learn::application::organizations::list_organization_courses::{
    OrganizationCourseListError, OrganizationCourseListOutput, OrganizationCourseListQuery,
    OrganizationCourseListUseCase, OrganizationCourseSummaryOutput,
};
use rust_learn::application::organizations::list_organization_member_audit::{
    OrganizationMemberAuditError, OrganizationMemberAuditEventOutput, OrganizationMemberAuditQuery,
    OrganizationMemberAuditUseCase,
};
use rust_learn::application::organizations::list_organization_members::{
    OrganizationMemberListError, OrganizationMemberListOutput, OrganizationMemberListQuery,
    OrganizationMemberListUseCase, OrganizationMemberOperatorPermissionsOutput,
    OrganizationMemberOrganizationOutput,
};

struct RouteOnlyOrganizationCourseListUseCase;
struct RouteOnlyOrganizationMemberAuditUseCase;
struct RouteOnlyOrganizationMemberListUseCase;

pub fn organization_course_list_data() -> web::Data<Arc<dyn OrganizationCourseListUseCase>> {
    web::Data::new(
        Arc::new(RouteOnlyOrganizationCourseListUseCase) as Arc<dyn OrganizationCourseListUseCase>
    )
}

pub fn organization_member_list_data() -> web::Data<Arc<dyn OrganizationMemberListUseCase>> {
    web::Data::new(
        Arc::new(RouteOnlyOrganizationMemberListUseCase) as Arc<dyn OrganizationMemberListUseCase>
    )
}

pub fn organization_member_audit_data() -> web::Data<Arc<dyn OrganizationMemberAuditUseCase>> {
    web::Data::new(Arc::new(RouteOnlyOrganizationMemberAuditUseCase)
        as Arc<dyn OrganizationMemberAuditUseCase>)
}

impl OrganizationCourseListUseCase for RouteOnlyOrganizationCourseListUseCase {
    fn list_organization_courses(
        &self,
        query: OrganizationCourseListQuery,
    ) -> BoxFuture<'_, Result<OrganizationCourseListOutput, OrganizationCourseListError>> {
        ready(Ok(OrganizationCourseListOutput {
            organization: OrganizationCourseSummaryOutput {
                id: query.organization_id,
                name: "Route smoke".to_string(),
            },
            courses: Vec::new(),
            total: 0,
            limit: query.limit,
            offset: query.offset,
            search: query.search,
            lifecycle_status: query.lifecycle_status,
            reward_available: query.reward_available,
        }))
        .boxed()
    }
}

impl OrganizationMemberAuditUseCase for RouteOnlyOrganizationMemberAuditUseCase {
    fn list_member_audit(
        &self,
        query: OrganizationMemberAuditQuery,
    ) -> BoxFuture<'_, Result<Vec<OrganizationMemberAuditEventOutput>, OrganizationMemberAuditError>>
    {
        ready(Ok(vec![OrganizationMemberAuditEventOutput {
            id: 1,
            organization_id: query.organization_id,
            actor_user_id: Some(query.actor_user_id),
            target_user_id: query.target_user_id,
            event_type: "route_smoke".to_string(),
            role_name: None,
            reason: None,
            created_at: Utc::now(),
        }]))
        .boxed()
    }
}

impl OrganizationMemberListUseCase for RouteOnlyOrganizationMemberListUseCase {
    fn list_organization_members(
        &self,
        query: OrganizationMemberListQuery,
    ) -> BoxFuture<'_, Result<OrganizationMemberListOutput, OrganizationMemberListError>> {
        ready(Ok(OrganizationMemberListOutput {
            organization: OrganizationMemberOrganizationOutput {
                id: query.organization_id,
                name: "Route smoke".to_string(),
            },
            members: Vec::new(),
            total: 0,
            limit: query.limit,
            offset: query.offset,
            search: query.search,
            role: query.role,
            permission: query.permission,
            operator_permissions: OrganizationMemberOperatorPermissionsOutput {
                can_view_members: true,
                can_invite_members: false,
                can_manage_members: false,
                can_assign_roles: false,
                can_manage_settings: false,
            },
        }))
        .boxed()
    }
}
