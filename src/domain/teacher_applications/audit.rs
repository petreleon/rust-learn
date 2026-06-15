use std::fmt;

use super::status::{
    TEACHER_APPLICATION_STATUS_APPROVED, TEACHER_APPLICATION_STATUS_NEEDS_CHANGES,
    TEACHER_APPLICATION_STATUS_REJECTED, TEACHER_APPLICATION_STATUS_SUBMITTED,
};

pub const TEACHER_APPLICATION_EVENT_ORGANIZATION_NOMINATED: &str = "organization_nominated";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TeacherApplicationAuditEventTypeError {
    InvalidEventType(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TeacherApplicationAuditEventType {
    Submitted,
    OrganizationNominated,
    Approved,
    NeedsChanges,
    Rejected,
}

impl TeacherApplicationAuditEventType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Submitted => TEACHER_APPLICATION_STATUS_SUBMITTED,
            Self::OrganizationNominated => TEACHER_APPLICATION_EVENT_ORGANIZATION_NOMINATED,
            Self::Approved => TEACHER_APPLICATION_STATUS_APPROVED,
            Self::NeedsChanges => TEACHER_APPLICATION_STATUS_NEEDS_CHANGES,
            Self::Rejected => TEACHER_APPLICATION_STATUS_REJECTED,
        }
    }

    pub fn parse(event_type: &str) -> Result<Self, TeacherApplicationAuditEventTypeError> {
        match event_type {
            TEACHER_APPLICATION_STATUS_SUBMITTED => Ok(Self::Submitted),
            TEACHER_APPLICATION_EVENT_ORGANIZATION_NOMINATED => Ok(Self::OrganizationNominated),
            TEACHER_APPLICATION_STATUS_APPROVED => Ok(Self::Approved),
            TEACHER_APPLICATION_STATUS_NEEDS_CHANGES => Ok(Self::NeedsChanges),
            TEACHER_APPLICATION_STATUS_REJECTED => Ok(Self::Rejected),
            _ => Err(TeacherApplicationAuditEventTypeError::InvalidEventType(
                "unsupported teacher application audit event type".to_string(),
            )),
        }
    }

    pub fn normalize(event_type: &str) -> Result<Self, TeacherApplicationAuditEventTypeError> {
        Self::parse(&event_type.trim().to_ascii_lowercase())
    }
}

impl AsRef<str> for TeacherApplicationAuditEventType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for TeacherApplicationAuditEventTypeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidEventType(message) => formatter.write_str(message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_known_audit_events() {
        assert_eq!(
            TeacherApplicationAuditEventType::parse("submitted").unwrap(),
            TeacherApplicationAuditEventType::Submitted
        );
        assert_eq!(
            TeacherApplicationAuditEventType::parse("organization_nominated").unwrap(),
            TeacherApplicationAuditEventType::OrganizationNominated
        );
        assert_eq!(
            TeacherApplicationAuditEventType::parse("approved").unwrap(),
            TeacherApplicationAuditEventType::Approved
        );
        assert_eq!(
            TeacherApplicationAuditEventType::parse("needs_changes").unwrap(),
            TeacherApplicationAuditEventType::NeedsChanges
        );
        assert_eq!(
            TeacherApplicationAuditEventType::parse("rejected").unwrap(),
            TeacherApplicationAuditEventType::Rejected
        );
    }

    #[test]
    fn normalizes_external_event_text() {
        assert_eq!(
            TeacherApplicationAuditEventType::normalize(" APPROVED ").unwrap(),
            TeacherApplicationAuditEventType::Approved
        );
    }

    #[test]
    fn rejects_unknown_audit_events() {
        assert!(TeacherApplicationAuditEventType::parse("pending").is_err());
        assert!(TeacherApplicationAuditEventType::parse("").is_err());
    }
}
