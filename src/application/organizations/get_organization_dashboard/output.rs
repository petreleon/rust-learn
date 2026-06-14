#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationDashboardOutput {
    pub organization: OrganizationDashboardOrganizationOutput,
    pub health: OrganizationDashboardHealthOutput,
    pub members: OrganizationDashboardMemberSummaryOutput,
    pub courses: OrganizationDashboardCourseSummaryOutput,
    pub teacher_applications: OrganizationDashboardTeacherApplicationSummaryOutput,
    pub rewards: OrganizationDashboardRewardSummaryOutput,
    pub wallet: OrganizationDashboardWalletSummaryOutput,
    pub operator_permissions: OrganizationDashboardOperatorPermissionsOutput,
    pub alerts: Vec<OrganizationDashboardAlertOutput>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationDashboardOrganizationOutput {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationDashboardHealthOutput {
    pub status: String,
    pub alert_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationDashboardMemberSummaryOutput {
    pub available: bool,
    pub missing_permissions: Vec<String>,
    pub total: i64,
    pub verified_email_count: i64,
    pub kyc_ready_count: i64,
    pub delegated_permission_count: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationDashboardCourseSummaryOutput {
    pub available: bool,
    pub missing_permissions: Vec<String>,
    pub total: i64,
    pub draft: i64,
    pub submitted: i64,
    pub needs_changes: i64,
    pub approved: i64,
    pub published: i64,
    pub suspended: i64,
    pub archived: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationDashboardTeacherApplicationSummaryOutput {
    pub available: bool,
    pub missing_permissions: Vec<String>,
    pub total: i64,
    pub submitted: i64,
    pub needs_changes: i64,
    pub approved: i64,
    pub rejected: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationDashboardRewardSummaryOutput {
    pub available: bool,
    pub missing_permissions: Vec<String>,
    pub reward_candidate_count: i64,
    pub approved_reward_count: i64,
    pub approved_amount_total: String,
    pub failed_count: i64,
    pub needs_reconciliation_count: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationDashboardWalletSummaryOutput {
    pub available: bool,
    pub missing_permissions: Vec<String>,
    pub wallet_count: i64,
    pub balance_total: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationDashboardOperatorPermissionsOutput {
    pub can_view_dashboard: bool,
    pub can_view_members: bool,
    pub can_view_courses: bool,
    pub can_view_reports: bool,
    pub can_view_teacher_applications: bool,
    pub can_nominate_teachers: bool,
    pub can_manage_wallets: bool,
    pub can_manage_reward_budget: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationDashboardAlertOutput {
    pub severity: String,
    pub kind: String,
    pub message: String,
    pub action_label: Option<String>,
    pub action_href: Option<String>,
}
