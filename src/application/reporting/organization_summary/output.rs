#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationSummaryOutput {
    pub organization_id: i32,
    pub organization_name: String,
    pub course_count: i64,
    pub member_count: i64,
    pub wallet_count: i64,
    pub course_role_assignment_count: i64,
}
