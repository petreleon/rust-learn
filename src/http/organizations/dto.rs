use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct AssignRoleRequest {
    pub(super) role_name: String,
}

#[derive(Deserialize)]
pub(super) struct OrganizationCourseListParams {
    pub(super) search: Option<String>,
    pub(super) lifecycle_status: Option<String>,
    pub(super) reward_available: Option<bool>,
    pub(super) limit: Option<i64>,
    pub(super) offset: Option<i64>,
}

#[derive(Deserialize)]
pub(super) struct OrganizationMemberListParams {
    pub(super) search: Option<String>,
    pub(super) role: Option<String>,
    pub(super) permission: Option<String>,
    pub(super) limit: Option<i64>,
    pub(super) offset: Option<i64>,
}

#[derive(Deserialize)]
pub(super) struct OrganizationTeacherApplicationsParams {
    pub(super) status: Option<String>,
    pub(super) search: Option<String>,
    pub(super) limit: Option<i64>,
    pub(super) offset: Option<i64>,
}

#[derive(Deserialize)]
pub(super) struct CreateOrganizationRequest {
    pub(super) name: String,
    pub(super) website_link: Option<String>,
    pub(super) profile_url: Option<String>,
    pub(super) course_ids: Option<Vec<i32>>,
}

#[derive(Deserialize)]
pub(super) struct AddMemberRequest {
    pub(super) email: String,
    pub(super) role_name: Option<String>,
}
