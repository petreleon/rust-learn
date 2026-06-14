use crate::application::organizations::remove_organization_member::OrganizationMemberRemovalError;

pub fn map_member_removal_error(error: diesel::result::Error) -> OrganizationMemberRemovalError {
    match error {
        diesel::result::Error::NotFound => OrganizationMemberRemovalError::NotFound,
        other => OrganizationMemberRemovalError::Database(other.to_string()),
    }
}
