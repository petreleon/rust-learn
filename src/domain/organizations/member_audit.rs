use std::fmt;

pub const ORGANIZATION_MEMBER_AUDIT_EVENT_MEMBER_INVITED: &str = "member_invited";
pub const ORGANIZATION_MEMBER_AUDIT_EVENT_MEMBER_REMOVED: &str = "member_removed";
pub const ORGANIZATION_MEMBER_AUDIT_EVENT_ROLE_ASSIGNED: &str = "role_assigned";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrganizationMemberAuditEventType {
    MemberInvited,
    MemberRemoved,
    RoleAssigned,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationMemberAuditEventTypeParseError {
    value: String,
}

impl OrganizationMemberAuditEventType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MemberInvited => ORGANIZATION_MEMBER_AUDIT_EVENT_MEMBER_INVITED,
            Self::MemberRemoved => ORGANIZATION_MEMBER_AUDIT_EVENT_MEMBER_REMOVED,
            Self::RoleAssigned => ORGANIZATION_MEMBER_AUDIT_EVENT_ROLE_ASSIGNED,
        }
    }

    pub fn parse(value: &str) -> Result<Self, OrganizationMemberAuditEventTypeParseError> {
        match value {
            ORGANIZATION_MEMBER_AUDIT_EVENT_MEMBER_INVITED => Ok(Self::MemberInvited),
            ORGANIZATION_MEMBER_AUDIT_EVENT_MEMBER_REMOVED => Ok(Self::MemberRemoved),
            ORGANIZATION_MEMBER_AUDIT_EVENT_ROLE_ASSIGNED => Ok(Self::RoleAssigned),
            other => Err(OrganizationMemberAuditEventTypeParseError {
                value: other.to_string(),
            }),
        }
    }
}

impl fmt::Display for OrganizationMemberAuditEventType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for OrganizationMemberAuditEventTypeParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "unknown organization member audit event type '{}'",
            self.value
        )
    }
}

#[cfg(test)]
mod tests {
    use super::OrganizationMemberAuditEventType;

    #[test]
    fn parses_known_member_audit_event_types() {
        assert_eq!(
            OrganizationMemberAuditEventType::parse("member_invited").unwrap(),
            OrganizationMemberAuditEventType::MemberInvited
        );
        assert_eq!(
            OrganizationMemberAuditEventType::parse("member_removed").unwrap(),
            OrganizationMemberAuditEventType::MemberRemoved
        );
        assert_eq!(
            OrganizationMemberAuditEventType::parse("role_assigned").unwrap(),
            OrganizationMemberAuditEventType::RoleAssigned
        );
    }

    #[test]
    fn rejects_unknown_member_audit_event_types() {
        assert!(OrganizationMemberAuditEventType::parse("role_revoked").is_err());
    }
}
