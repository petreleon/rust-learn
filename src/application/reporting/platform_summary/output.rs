#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformSummaryOutput {
    pub total_users: i64,
    pub total_organizations: i64,
    pub total_courses: i64,
    pub total_wallets: i64,
    pub total_notifications: i64,
}
