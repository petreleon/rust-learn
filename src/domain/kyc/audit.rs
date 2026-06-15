use std::fmt;

pub const KYC_AUDIT_EVENT_REVIEW_DECISION: &str = "review_decision";
pub const KYC_AUDIT_EVENT_SUBMITTED: &str = "submitted";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KycAuditEventType {
    Submitted,
    ReviewDecision,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KycAuditEventTypeParseError {
    value: String,
}

impl KycAuditEventType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Submitted => KYC_AUDIT_EVENT_SUBMITTED,
            Self::ReviewDecision => KYC_AUDIT_EVENT_REVIEW_DECISION,
        }
    }

    pub fn parse(value: &str) -> Result<Self, KycAuditEventTypeParseError> {
        match value {
            KYC_AUDIT_EVENT_SUBMITTED => Ok(Self::Submitted),
            KYC_AUDIT_EVENT_REVIEW_DECISION => Ok(Self::ReviewDecision),
            other => Err(KycAuditEventTypeParseError {
                value: other.to_string(),
            }),
        }
    }
}

impl fmt::Display for KycAuditEventType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for KycAuditEventTypeParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "unknown KYC audit event type '{}'", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::KycAuditEventType;

    #[test]
    fn parses_known_kyc_audit_event_types() {
        assert_eq!(
            KycAuditEventType::parse("submitted").unwrap(),
            KycAuditEventType::Submitted
        );
        assert_eq!(
            KycAuditEventType::parse("review_decision").unwrap(),
            KycAuditEventType::ReviewDecision
        );
    }

    #[test]
    fn rejects_unknown_kyc_audit_event_types() {
        assert!(KycAuditEventType::parse("member_invited").is_err());
    }
}
