use serde::Serialize;

use crate::application::learning::list_course_organizations::CourseOrganizationOutput;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CourseOrganizationResponse {
    pub id: i32,
    pub name: String,
    pub website_link: Option<String>,
    pub profile_url: Option<String>,
}

impl From<CourseOrganizationOutput> for CourseOrganizationResponse {
    fn from(organization: CourseOrganizationOutput) -> Self {
        Self {
            id: organization.id,
            name: organization.name,
            website_link: organization.website_link,
            profile_url: organization.profile_url,
        }
    }
}
