#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestCourseJoinCommand {
    pub actor_user_id: i32,
    pub course_id: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecideCourseJoinCommand {
    pub reviewer_user_id: i32,
    pub course_id: i32,
    pub request_id: i64,
    pub status: String,
    pub decision_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveCourseEnrollmentCommand {
    pub actor_user_id: i32,
    pub course_id: i32,
    pub target_user_id: i32,
}
