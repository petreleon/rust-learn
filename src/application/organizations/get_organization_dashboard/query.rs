#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrganizationDashboardQuery {
    pub actor_user_id: i32,
    pub organization_id: i32,
}
