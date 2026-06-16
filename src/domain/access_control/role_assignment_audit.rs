use std::fmt;

pub const PLATFORM_ROLE_AUDIT_EVENT_ROLE_ASSIGNED: &str = "role_assigned";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformRoleAssignmentAuditEventType {
    RoleAssigned,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformRoleAssignmentAuditEventTypeParseError {
    value: String,
}

impl PlatformRoleAssignmentAuditEventType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RoleAssigned => PLATFORM_ROLE_AUDIT_EVENT_ROLE_ASSIGNED,
        }
    }

    pub fn parse(value: &str) -> Result<Self, PlatformRoleAssignmentAuditEventTypeParseError> {
        match value {
            PLATFORM_ROLE_AUDIT_EVENT_ROLE_ASSIGNED => Ok(Self::RoleAssigned),
            other => Err(PlatformRoleAssignmentAuditEventTypeParseError {
                value: other.to_string(),
            }),
        }
    }
}

impl fmt::Display for PlatformRoleAssignmentAuditEventType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for PlatformRoleAssignmentAuditEventTypeParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "unknown platform role assignment audit event type '{}'",
            self.value
        )
    }
}

#[cfg(test)]
mod tests {
    use super::PlatformRoleAssignmentAuditEventType;

    #[test]
    fn parses_known_platform_role_assignment_audit_event_types() {
        assert_eq!(
            PlatformRoleAssignmentAuditEventType::parse("role_assigned").unwrap(),
            PlatformRoleAssignmentAuditEventType::RoleAssigned
        );
    }

    #[test]
    fn rejects_unknown_platform_role_assignment_audit_event_types() {
        assert!(PlatformRoleAssignmentAuditEventType::parse("role_revoked").is_err());
    }
}
