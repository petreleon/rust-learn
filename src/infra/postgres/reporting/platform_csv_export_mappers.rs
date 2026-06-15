use crate::application::reporting::platform_csv_exports::PlatformCsvExportError;

pub(super) fn map_diesel_error(error: diesel::result::Error) -> PlatformCsvExportError {
    PlatformCsvExportError::Database(error.to_string())
}
