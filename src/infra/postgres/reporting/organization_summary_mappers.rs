use crate::application::reporting::organization_summary::OrganizationSummaryError;

pub(super) fn map_diesel_error(error: diesel::result::Error) -> OrganizationSummaryError {
    match error {
        diesel::result::Error::NotFound => OrganizationSummaryError::NotFound,
        other => OrganizationSummaryError::Database(other.to_string()),
    }
}
