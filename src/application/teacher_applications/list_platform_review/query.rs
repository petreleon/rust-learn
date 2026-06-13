#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TeacherApplicationPlatformReviewQuery {
    pub actor_user_id: i32,
    pub status: Option<String>,
    pub search: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
