use serde::Serialize;

use crate::application::learning::get_teacher_course_enrollment_workspace::TeacherCourseEnrollmentWorkspaceOutput;

use super::teacher_course_dashboard::TeacherCourseDashboardItemResponse;
use super::teacher_course_enrollment::{
    TeacherCourseJoinRequestPageResponse, TeacherCourseRewardEligibilitySummaryResponse,
    TeacherCourseRosterPageResponse,
};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TeacherCourseEnrollmentWorkspaceResponse {
    pub course: TeacherCourseDashboardItemResponse,
    pub teacher_roles: Vec<String>,
    pub join_requests: TeacherCourseJoinRequestPageResponse,
    pub roster: TeacherCourseRosterPageResponse,
    pub progress_supported: bool,
    pub reward_eligibility_supported: bool,
    pub reward_eligibility: TeacherCourseRewardEligibilitySummaryResponse,
}

impl From<TeacherCourseEnrollmentWorkspaceOutput> for TeacherCourseEnrollmentWorkspaceResponse {
    fn from(output: TeacherCourseEnrollmentWorkspaceOutput) -> Self {
        Self {
            course: output.course.into(),
            teacher_roles: output.teacher_roles,
            join_requests: output.join_requests.into(),
            roster: output.roster.into(),
            progress_supported: output.progress_supported,
            reward_eligibility_supported: output.reward_eligibility_supported,
            reward_eligibility: output.reward_eligibility.into(),
        }
    }
}
