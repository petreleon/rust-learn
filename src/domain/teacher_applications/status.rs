use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TeacherApplicationStatusError {
    InvalidStatus(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TeacherApplicationStatus {
    Approved,
    NeedsChanges,
    Rejected,
    Submitted,
}

pub const TEACHER_APPLICATION_STATUS_APPROVED: &str = "approved";
pub const TEACHER_APPLICATION_STATUS_NEEDS_CHANGES: &str = "needs_changes";
pub const TEACHER_APPLICATION_STATUS_REJECTED: &str = "rejected";
pub const TEACHER_APPLICATION_STATUS_SUBMITTED: &str = "submitted";

impl TeacherApplicationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Approved => TEACHER_APPLICATION_STATUS_APPROVED,
            Self::NeedsChanges => TEACHER_APPLICATION_STATUS_NEEDS_CHANGES,
            Self::Rejected => TEACHER_APPLICATION_STATUS_REJECTED,
            Self::Submitted => TEACHER_APPLICATION_STATUS_SUBMITTED,
        }
    }

    pub fn parse(status: &str) -> Result<Self, TeacherApplicationStatusError> {
        match status {
            TEACHER_APPLICATION_STATUS_APPROVED => Ok(Self::Approved),
            TEACHER_APPLICATION_STATUS_NEEDS_CHANGES => Ok(Self::NeedsChanges),
            TEACHER_APPLICATION_STATUS_REJECTED => Ok(Self::Rejected),
            TEACHER_APPLICATION_STATUS_SUBMITTED => Ok(Self::Submitted),
            _ => Err(TeacherApplicationStatusError::InvalidStatus(
                "unsupported teacher application status".to_string(),
            )),
        }
    }

    pub fn normalize(status: &str) -> Result<Self, TeacherApplicationStatusError> {
        Self::parse(&status.trim().to_ascii_lowercase())
    }
}

pub fn normalize_status(status: &str) -> Result<String, TeacherApplicationStatusError> {
    TeacherApplicationStatus::normalize(status).map(|status| status.as_str().to_string())
}

pub fn normalize_decision_status(status: &str) -> Result<String, TeacherApplicationStatusError> {
    let normalized = TeacherApplicationStatus::normalize(status)?;
    match normalized {
        TeacherApplicationStatus::Approved
        | TeacherApplicationStatus::NeedsChanges
        | TeacherApplicationStatus::Rejected => Ok(normalized.as_str().to_string()),
        TeacherApplicationStatus::Submitted => Err(TeacherApplicationStatusError::InvalidStatus(
            "decision status must be approved, rejected, or needs_changes".to_string(),
        )),
    }
}

pub fn normalize_optional_status(
    status: Option<String>,
) -> Result<Option<String>, TeacherApplicationStatusError> {
    status.as_deref().map(normalize_status).transpose()
}

impl fmt::Display for TeacherApplicationStatusError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidStatus(message) => formatter.write_str(message),
        }
    }
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
        assert_eq!(
            TeacherApplicationStatus::parse("approved").unwrap(),
            TeacherApplicationStatus::Approved
        );
    }

    #[test]
    fn rejects_unknown_statuses() {
        assert!(normalize_status("pending").is_err());
        assert!(normalize_status("").is_err());
    }

    #[test]
    fn normalizes_decision_statuses() {
        assert_eq!(normalize_decision_status(" APPROVED ").unwrap(), "approved");
        assert_eq!(
            normalize_decision_status("needs_changes").unwrap(),
            "needs_changes"
        );
        assert_eq!(normalize_decision_status("rejected").unwrap(), "rejected");
    }

    #[test]
    fn rejects_submitted_as_decision_status() {
        assert!(normalize_decision_status("submitted").is_err());
    }
}
