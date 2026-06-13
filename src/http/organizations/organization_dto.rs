use serde::Serialize;

use crate::application::organizations::manage_organizations::OrganizationOutput;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationResponse {
    pub id: i32,
    pub name: String,
    pub website_link: Option<String>,
    pub profile_url: Option<String>,
}

impl From<OrganizationOutput> for OrganizationResponse {
    fn from(organization: OrganizationOutput) -> Self {
        Self {
            id: organization.id,
            name: organization.name,
            website_link: organization.website_link,
            profile_url: organization.profile_url,
        }
    }
}
