use std::sync::Arc;

use actix_web::web;
use futures::future::{ready, BoxFuture, FutureExt};
use rust_learn::application::organizations::get_organization_dashboard::{
    OrganizationDashboardCourseSummaryOutput, OrganizationDashboardError,
    OrganizationDashboardHealthOutput, OrganizationDashboardMemberSummaryOutput,
    OrganizationDashboardOperatorPermissionsOutput, OrganizationDashboardOrganizationOutput,
    OrganizationDashboardOutput, OrganizationDashboardQuery,
    OrganizationDashboardRewardSummaryOutput, OrganizationDashboardTeacherApplicationSummaryOutput,
    OrganizationDashboardUseCase, OrganizationDashboardWalletSummaryOutput,
};

struct RouteOnlyOrganizationDashboardUseCase;

pub fn organization_dashboard_data() -> web::Data<Arc<dyn OrganizationDashboardUseCase>> {
    web::Data::new(
        Arc::new(RouteOnlyOrganizationDashboardUseCase) as Arc<dyn OrganizationDashboardUseCase>
    )
}

impl OrganizationDashboardUseCase for RouteOnlyOrganizationDashboardUseCase {
    fn get_organization_dashboard(
        &self,
        query: OrganizationDashboardQuery,
    ) -> BoxFuture<'_, Result<OrganizationDashboardOutput, OrganizationDashboardError>> {
        ready(Ok(OrganizationDashboardOutput {
            organization: OrganizationDashboardOrganizationOutput {
                id: query.organization_id,
                name: "Route smoke".to_string(),
            },
            health: OrganizationDashboardHealthOutput {
                status: "active".to_string(),
                alert_count: 0,
            },
            members: member_summary(),
            courses: course_summary(),
            teacher_applications: teacher_application_summary(),
            rewards: reward_summary(),
            wallet: wallet_summary(),
            operator_permissions: operator_permissions(),
            alerts: Vec::new(),
        }))
        .boxed()
    }
}

fn member_summary() -> OrganizationDashboardMemberSummaryOutput {
    OrganizationDashboardMemberSummaryOutput {
        available: true,
        missing_permissions: vec![],
        total: 0,
        verified_email_count: 0,
        kyc_ready_count: 0,
        delegated_permission_count: 0,
    }
}

fn course_summary() -> OrganizationDashboardCourseSummaryOutput {
    OrganizationDashboardCourseSummaryOutput {
        available: true,
        missing_permissions: vec![],
        total: 0,
        draft: 0,
        submitted: 0,
        needs_changes: 0,
        approved: 0,
        published: 0,
        suspended: 0,
        archived: 0,
    }
}

fn teacher_application_summary() -> OrganizationDashboardTeacherApplicationSummaryOutput {
    OrganizationDashboardTeacherApplicationSummaryOutput {
        available: false,
        missing_permissions: vec![],
        total: 0,
        submitted: 0,
        needs_changes: 0,
        approved: 0,
        rejected: 0,
    }
}

fn reward_summary() -> OrganizationDashboardRewardSummaryOutput {
    OrganizationDashboardRewardSummaryOutput {
        available: false,
        missing_permissions: vec![],
        reward_candidate_count: 0,
        approved_reward_count: 0,
        approved_amount_total: "0".to_string(),
        failed_count: 0,
        needs_reconciliation_count: 0,
    }
}

fn wallet_summary() -> OrganizationDashboardWalletSummaryOutput {
    OrganizationDashboardWalletSummaryOutput {
        available: false,
        missing_permissions: vec![],
        wallet_count: 0,
        balance_total: "0".to_string(),
    }
}

fn operator_permissions() -> OrganizationDashboardOperatorPermissionsOutput {
    OrganizationDashboardOperatorPermissionsOutput {
        can_view_dashboard: true,
        can_view_members: true,
        can_view_courses: true,
        can_view_reports: false,
        can_view_teacher_applications: false,
        can_nominate_teachers: false,
        can_manage_wallets: false,
        can_manage_reward_budget: false,
    }
}
