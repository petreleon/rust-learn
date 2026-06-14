use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmitAssessmentAttemptCommand {
    pub course_id: i32,
    pub assessment_id: i32,
    pub user_id: i32,
    pub answers: HashMap<i32, String>,
}
