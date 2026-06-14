#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CourseRewardCandidatesQuery {
    pub status: Option<String>,
    pub student_user_id: Option<i32>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseRewardCandidatesFilter {
    pub course_id: i32,
    pub student_user_id: Option<i32>,
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
