use futures::future::{ready, BoxFuture, FutureExt};

use crate::application::organizations::get_organization_dashboard::{
    OrganizationDashboardCourseSummaryOutput, OrganizationDashboardError,
    OrganizationDashboardMemberSummaryOutput, OrganizationDashboardOperatorPermissionsOutput,
    OrganizationDashboardOrganizationOutput, OrganizationDashboardRewardSummaryOutput,
    OrganizationDashboardStore, OrganizationDashboardTeacherApplicationSummaryOutput,
    OrganizationDashboardWalletSummaryOutput,
};

use super::fake_store_summaries::{reward_summary, teacher_application_summary, wallet_summary};

pub struct FakeOrganizationDashboardStore {
    can_view: bool,
    permissions: OrganizationDashboardOperatorPermissionsOutput,
    pub loaded_permissions: bool,
    pub loaded_members: bool,
    pub loaded_courses: bool,
    pub loaded_teacher_applications: bool,
    pub loaded_rewards: bool,
    pub loaded_wallet: bool,
}

impl FakeOrganizationDashboardStore {
    pub fn full_access() -> Self {
        Self {
            can_view: true,
            permissions: OrganizationDashboardOperatorPermissionsOutput {
                can_view_dashboard: true,
                can_view_members: true,
                can_view_courses: true,
                can_view_reports: true,
                can_view_teacher_applications: true,
                can_nominate_teachers: true,
                can_manage_wallets: true,
                can_manage_reward_budget: true,
            },
            loaded_permissions: false,
            loaded_members: false,
            loaded_courses: false,
            loaded_teacher_applications: false,
            loaded_rewards: false,
            loaded_wallet: false,
        }
    }

    pub fn limited() -> Self {
        Self {
            permissions: OrganizationDashboardOperatorPermissionsOutput {
                can_view_dashboard: true,
                can_view_members: true,
                can_view_courses: true,
                can_view_reports: false,
                can_view_teacher_applications: false,
                can_nominate_teachers: false,
                can_manage_wallets: false,
                can_manage_reward_budget: false,
            },
            ..Self::full_access()
        }
    }

    pub fn denied() -> Self {
        Self {
            can_view: false,
            ..Self::full_access()
        }
    }
}

impl OrganizationDashboardStore for FakeOrganizationDashboardStore {
    fn load_organization(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<OrganizationDashboardOrganizationOutput, OrganizationDashboardError>>
    {
        ready(Ok(OrganizationDashboardOrganizationOutput {
            id: organization_id,
            name: "Org".to_string(),
        }))
        .boxed()
    }

    fn can_view_dashboard(
        &mut self,
        _actor_user_id: i32,
        _organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, OrganizationDashboardError>> {
        ready(Ok(self.can_view)).boxed()
    }

    fn load_operator_permissions(
        &mut self,
        _actor_user_id: i32,
        _organization_id: i32,
    ) -> BoxFuture<
        '_,
        Result<OrganizationDashboardOperatorPermissionsOutput, OrganizationDashboardError>,
    > {
        self.loaded_permissions = true;
        ready(Ok(self.permissions.clone())).boxed()
    }

    fn load_member_summary(
        &mut self,
        _organization_id: i32,
    ) -> BoxFuture<'_, Result<OrganizationDashboardMemberSummaryOutput, OrganizationDashboardError>>
    {
        self.loaded_members = true;
        ready(Ok(member_summary())).boxed()
    }

    fn load_course_summary(
        &mut self,
        _organization_id: i32,
    ) -> BoxFuture<'_, Result<OrganizationDashboardCourseSummaryOutput, OrganizationDashboardError>>
    {
        self.loaded_courses = true;
        ready(Ok(course_summary())).boxed()
    }

    fn load_teacher_application_summary(
        &mut self,
        _organization_id: i32,
    ) -> BoxFuture<
        '_,
        Result<OrganizationDashboardTeacherApplicationSummaryOutput, OrganizationDashboardError>,
    > {
        self.loaded_teacher_applications = true;
        ready(Ok(teacher_application_summary())).boxed()
    }

    fn load_reward_summary(
        &mut self,
        _organization_id: i32,
    ) -> BoxFuture<'_, Result<OrganizationDashboardRewardSummaryOutput, OrganizationDashboardError>>
    {
        self.loaded_rewards = true;
        ready(Ok(reward_summary())).boxed()
    }

    fn load_wallet_summary(
        &mut self,
        _organization_id: i32,
    ) -> BoxFuture<'_, Result<OrganizationDashboardWalletSummaryOutput, OrganizationDashboardError>>
    {
        self.loaded_wallet = true;
        ready(Ok(wallet_summary())).boxed()
    }
}

fn member_summary() -> OrganizationDashboardMemberSummaryOutput {
    OrganizationDashboardMemberSummaryOutput {
        available: true,
        missing_permissions: vec![],
        total: 2,
        verified_email_count: 2,
        kyc_ready_count: 1,
        delegated_permission_count: 0,
    }
}

fn course_summary() -> OrganizationDashboardCourseSummaryOutput {
    OrganizationDashboardCourseSummaryOutput {
        available: true,
        missing_permissions: vec![],
        total: 1,
        draft: 0,
        submitted: 0,
        needs_changes: 1,
        approved: 0,
        published: 0,
        suspended: 0,
        archived: 0,
    }
}
