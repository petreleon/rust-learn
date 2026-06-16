use std::fmt;

pub const REWARD_POLICY_AUDIT_EVENT_CREATED: &str = "created";
pub const REWARD_POLICY_AUDIT_EVENT_ACTIVATED: &str = "activated";
pub const REWARD_POLICY_AUDIT_EVENT_DEACTIVATED: &str = "deactivated";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewardPolicyAuditEventType {
    Created,
    Activated,
    Deactivated,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyAuditEventTypeParseError {
    value: String,
}

impl RewardPolicyAuditEventType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Created => REWARD_POLICY_AUDIT_EVENT_CREATED,
            Self::Activated => REWARD_POLICY_AUDIT_EVENT_ACTIVATED,
            Self::Deactivated => REWARD_POLICY_AUDIT_EVENT_DEACTIVATED,
        }
    }

    pub fn parse(value: &str) -> Result<Self, PolicyAuditEventTypeParseError> {
        match value {
            REWARD_POLICY_AUDIT_EVENT_CREATED => Ok(Self::Created),
            REWARD_POLICY_AUDIT_EVENT_ACTIVATED => Ok(Self::Activated),
            REWARD_POLICY_AUDIT_EVENT_DEACTIVATED => Ok(Self::Deactivated),
            other => Err(PolicyAuditEventTypeParseError {
                value: other.to_string(),
            }),
        }
    }
}

impl fmt::Display for RewardPolicyAuditEventType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for PolicyAuditEventTypeParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "unknown reward policy audit event type '{}'",
            self.value
        )
    }
}
