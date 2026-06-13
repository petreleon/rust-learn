#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TeacherApplicationListQuery {
    pub actor_user_id: i32,
    pub status: Option<String>,
    pub applicant_user_id: Option<i32>,
    pub organization_sponsor_id: Option<i32>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
