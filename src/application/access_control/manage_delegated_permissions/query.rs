#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ListDelegatedPermissionsQuery {
    pub actor_user_id: i32,
    pub grantor_user_id: Option<i32>,
    pub grantee_user_id: Option<i32>,
    pub permission: Option<String>,
    pub scope_type: Option<String>,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub active: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
