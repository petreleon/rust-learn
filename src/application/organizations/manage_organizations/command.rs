#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationCreateCommand {
    pub name: String,
    pub website_link: Option<String>,
    pub profile_url: Option<String>,
    pub course_ids: Option<Vec<i32>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationUpdateCommand {
    pub organization_id: i32,
    pub name: Option<String>,
    pub website_link: Option<String>,
    pub profile_url: Option<String>,
}
