use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardItemOutput;
use crate::application::learning::teacher_course_enrollment::{
    TeacherCourseJoinRequestPageOutput, TeacherCourseRewardEligibilitySummaryOutput,
    TeacherCourseRosterPageOutput,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherCourseEnrollmentWorkspaceOutput {
    pub course: TeacherCourseDashboardItemOutput,
    pub teacher_roles: Vec<String>,
    pub join_requests: TeacherCourseJoinRequestPageOutput,
    pub roster: TeacherCourseRosterPageOutput,
    pub progress_supported: bool,
    pub reward_eligibility_supported: bool,
    pub reward_eligibility: TeacherCourseRewardEligibilitySummaryOutput,
}
