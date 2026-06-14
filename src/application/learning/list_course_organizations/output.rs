#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseOrganizationOutput {
    pub id: i32,
    pub name: String,
    pub website_link: Option<String>,
    pub profile_url: Option<String>,
}
