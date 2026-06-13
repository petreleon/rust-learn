#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TeacherApplicationStatusError {
    InvalidStatus(String),
}

pub const TEACHER_APPLICATION_STATUS_APPROVED: &str = "approved";
pub const TEACHER_APPLICATION_STATUS_NEEDS_CHANGES: &str = "needs_changes";
pub const TEACHER_APPLICATION_STATUS_REJECTED: &str = "rejected";
pub const TEACHER_APPLICATION_STATUS_SUBMITTED: &str = "submitted";

pub fn normalize_status(status: &str) -> Result<String, TeacherApplicationStatusError> {
    let normalized = status.trim().to_ascii_lowercase();
    match normalized.as_str() {
        TEACHER_APPLICATION_STATUS_APPROVED
        | TEACHER_APPLICATION_STATUS_NEEDS_CHANGES
        | TEACHER_APPLICATION_STATUS_REJECTED
        | TEACHER_APPLICATION_STATUS_SUBMITTED => Ok(normalized),
        _ => Err(TeacherApplicationStatusError::InvalidStatus(
            "unsupported teacher application status".to_string(),
        )),
    }
}

pub fn normalize_optional_status(
    status: Option<String>,
) -> Result<Option<String>, TeacherApplicationStatusError> {
    status.as_deref().map(normalize_status).transpose()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_known_statuses() {
        assert_eq!(normalize_status(" SUBMITTED ").unwrap(), "submitted");
        assert_eq!(normalize_status("needs_changes").unwrap(), "needs_changes");
        assert_eq!(normalize_status("approved").unwrap(), "approved");
        assert_eq!(normalize_status("rejected").unwrap(), "rejected");
    }

    #[test]
    fn rejects_unknown_statuses() {
        assert!(normalize_status("pending").is_err());
        assert!(normalize_status("").is_err());
    }
}
