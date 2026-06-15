use std::fmt;

pub const REWARD_FRAUD_BLOCK_SCOPE_TEACHER: &str = "teacher";
pub const REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION: &str = "organization";
pub const REWARD_FRAUD_BLOCK_SCOPE_COURSE: &str = "course";
pub const REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY: &str = "reward_policy";
pub const REWARD_FRAUD_BLOCK_EVENT_CREATED: &str = "created";
pub const REWARD_FRAUD_BLOCK_EVENT_REVOKED: &str = "revoked";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewardFraudBlockScope {
    Teacher,
    Organization,
    Course,
    RewardPolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewardFraudBlockAuditEventType {
    Created,
    Revoked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FraudBlockScopeParseError {
    value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FraudBlockAuditEventParseError {
    value: String,
}

impl RewardFraudBlockScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Teacher => REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
            Self::Organization => REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION,
            Self::Course => REWARD_FRAUD_BLOCK_SCOPE_COURSE,
            Self::RewardPolicy => REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY,
        }
    }

    pub fn parse(value: &str) -> Result<Self, FraudBlockScopeParseError> {
        match value {
            REWARD_FRAUD_BLOCK_SCOPE_TEACHER => Ok(Self::Teacher),
            REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION => Ok(Self::Organization),
            REWARD_FRAUD_BLOCK_SCOPE_COURSE => Ok(Self::Course),
            REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY => Ok(Self::RewardPolicy),
            other => Err(FraudBlockScopeParseError {
                value: other.to_string(),
            }),
        }
    }

    pub fn normalize(value: &str) -> Result<Self, FraudBlockScopeParseError> {
        Self::parse(value.trim())
    }

    pub fn matches_target(
        self,
        has_teacher: bool,
        has_organization: bool,
        has_course: bool,
        has_reward_policy: bool,
    ) -> bool {
        match self {
            Self::Teacher => has_teacher,
            Self::Organization => has_organization,
            Self::Course => has_course,
            Self::RewardPolicy => has_reward_policy,
        }
    }
}

impl RewardFraudBlockAuditEventType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Created => REWARD_FRAUD_BLOCK_EVENT_CREATED,
            Self::Revoked => REWARD_FRAUD_BLOCK_EVENT_REVOKED,
        }
    }

    pub fn parse(value: &str) -> Result<Self, FraudBlockAuditEventParseError> {
        match value {
            REWARD_FRAUD_BLOCK_EVENT_CREATED => Ok(Self::Created),
            REWARD_FRAUD_BLOCK_EVENT_REVOKED => Ok(Self::Revoked),
            other => Err(FraudBlockAuditEventParseError {
                value: other.to_string(),
            }),
        }
    }
}

impl fmt::Display for RewardFraudBlockScope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for RewardFraudBlockAuditEventType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for FraudBlockScopeParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "unknown reward fraud block scope '{}'",
            self.value
        )
    }
}

impl fmt::Display for FraudBlockAuditEventParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "unknown reward fraud block audit event type '{}'",
            self.value
        )
    }
}

pub fn normalize_scope_type(scope_type: &str) -> Option<String> {
    RewardFraudBlockScope::normalize(scope_type)
        .map(|scope| scope.as_str().to_string())
        .ok()
}

pub fn scope_matches_target(
    scope_type: &str,
    has_teacher: bool,
    has_organization: bool,
    has_course: bool,
    has_reward_policy: bool,
) -> bool {
    RewardFraudBlockScope::parse(scope_type)
        .map(|scope| {
            scope.matches_target(has_teacher, has_organization, has_course, has_reward_policy)
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests;
