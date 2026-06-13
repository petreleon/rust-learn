#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StudentRewardHistoryQuery {
    pub course_id: Option<i32>,
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudentRewardHistoryFilter {
    pub student_user_id: i32,
    pub course_id: Option<i32>,
    pub status: Option<String>,
    pub limit: i64,
    pub offset: i64,
}
