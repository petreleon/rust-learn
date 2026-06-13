use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationDashboardOrganizationResponse {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationDashboardHealthResponse {
    pub status: String,
    pub alert_count: usize,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationDashboardMemberSummaryResponse {
    pub available: bool,
    pub missing_permissions: Vec<String>,
    pub total: i64,
    pub verified_email_count: i64,
    pub kyc_ready_count: i64,
    pub delegated_permission_count: i64,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationDashboardCourseSummaryResponse {
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

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationDashboardTeacherApplicationSummaryResponse {
    pub available: bool,
    pub missing_permissions: Vec<String>,
    pub total: i64,
    pub submitted: i64,
    pub needs_changes: i64,
    pub approved: i64,
    pub rejected: i64,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationDashboardRewardSummaryResponse {
    pub available: bool,
    pub missing_permissions: Vec<String>,
    pub reward_candidate_count: i64,
    pub approved_reward_count: i64,
    pub approved_amount_total: String,
    pub failed_count: i64,
    pub needs_reconciliation_count: i64,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationDashboardWalletSummaryResponse {
    pub available: bool,
    pub missing_permissions: Vec<String>,
    pub wallet_count: i64,
    pub balance_total: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationDashboardOperatorPermissionsResponse {
    pub can_view_dashboard: bool,
    pub can_view_members: bool,
    pub can_view_courses: bool,
    pub can_view_reports: bool,
    pub can_view_teacher_applications: bool,
    pub can_nominate_teachers: bool,
    pub can_manage_wallets: bool,
    pub can_manage_reward_budget: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationDashboardAlertResponse {
    pub severity: String,
    pub kind: String,
    pub message: String,
    pub action_label: Option<String>,
    pub action_href: Option<String>,
}
