use crate::domain::teacher_applications::portfolio::TeacherApplicationPortfolioLinks;

#[derive(Debug, Clone, PartialEq)]
pub struct TeacherApplicationSubmission {
    pub applicant_user_id: i32,
    pub requested_scope: String,
    pub requested_organization_id: Option<i32>,
    pub requested_course_id: Option<i32>,
    pub experience_summary: String,
    pub organization_sponsor_id: Option<i32>,
    pub portfolio_links: TeacherApplicationPortfolioLinks,
    pub idempotency_key: Option<String>,
    pub status: String,
}
