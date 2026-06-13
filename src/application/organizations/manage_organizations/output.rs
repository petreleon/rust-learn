#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationOutput {
    pub id: i32,
    pub name: String,
    pub website_link: Option<String>,
    pub profile_url: Option<String>,
}
