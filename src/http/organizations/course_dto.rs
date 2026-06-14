use serde::Serialize;

use crate::application::organizations::list_organization_courses::OrganizationCourseListOutput;

use super::course_nested_dto::{
    OrganizationCourseListItemResponse, OrganizationCourseSummaryResponse,
};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationCourseListResponse {
    pub organization: OrganizationCourseSummaryResponse,
    pub courses: Vec<OrganizationCourseListItemResponse>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub search: Option<String>,
    pub lifecycle_status: Option<String>,
    pub reward_available: Option<bool>,
}

impl From<OrganizationCourseListOutput> for OrganizationCourseListResponse {
    fn from(output: OrganizationCourseListOutput) -> Self {
        Self {
            organization: output.organization.into(),
            courses: output.courses.into_iter().map(Into::into).collect(),
            total: output.total,
            limit: output.limit,
            offset: output.offset,
            search: output.search,
            lifecycle_status: output.lifecycle_status,
            reward_available: output.reward_available,
        }
    }
}
