use std::fmt;

pub const COURSE_TERMS_STATUS_SUBMITTED: &str = "submitted";
pub const COURSE_TERMS_STATUS_COUNTERED: &str = "countered";
pub const COURSE_TERMS_STATUS_ACCEPTED: &str = "accepted";
pub const COURSE_TERMS_STATUS_ACTIVE: &str = "active";
pub const COURSE_TERMS_STATUS_REJECTED: &str = "rejected";
pub const COURSE_TERMS_STATUS_WITHDRAWN: &str = "withdrawn";
pub const COURSE_TERMS_STATUS_SUPERSEDED: &str = "superseded";

pub const COURSE_TERMS_EVENT_PROPOSED: &str = "proposed";
pub const COURSE_TERMS_EVENT_COUNTERED: &str = "countered";
pub const COURSE_TERMS_EVENT_ACCEPTED: &str = "accepted";
pub const COURSE_TERMS_EVENT_ACTIVATED: &str = "activated";
pub const COURSE_TERMS_EVENT_REJECTED: &str = "rejected";
pub const COURSE_TERMS_EVENT_WITHDRAWN: &str = "withdrawn";
pub const COURSE_TERMS_EVENT_SUPERSEDED: &str = "superseded";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CourseCompletionTermsStatus {
    Submitted,
    Countered,
    Accepted,
    Active,
    Rejected,
    Withdrawn,
    Superseded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CourseCompletionTermsAuditEventType {
    Proposed,
    Countered,
    Accepted,
    Activated,
    Rejected,
    Withdrawn,
    Superseded,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseCompletionTermsParseError {
    value: String,
}

impl CourseCompletionTermsStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Submitted => COURSE_TERMS_STATUS_SUBMITTED,
            Self::Countered => COURSE_TERMS_STATUS_COUNTERED,
            Self::Accepted => COURSE_TERMS_STATUS_ACCEPTED,
            Self::Active => COURSE_TERMS_STATUS_ACTIVE,
            Self::Rejected => COURSE_TERMS_STATUS_REJECTED,
            Self::Withdrawn => COURSE_TERMS_STATUS_WITHDRAWN,
            Self::Superseded => COURSE_TERMS_STATUS_SUPERSEDED,
        }
    }

    pub fn parse(value: &str) -> Result<Self, CourseCompletionTermsParseError> {
        match value {
            COURSE_TERMS_STATUS_SUBMITTED => Ok(Self::Submitted),
            COURSE_TERMS_STATUS_COUNTERED => Ok(Self::Countered),
            COURSE_TERMS_STATUS_ACCEPTED => Ok(Self::Accepted),
            COURSE_TERMS_STATUS_ACTIVE => Ok(Self::Active),
            COURSE_TERMS_STATUS_REJECTED => Ok(Self::Rejected),
            COURSE_TERMS_STATUS_WITHDRAWN => Ok(Self::Withdrawn),
            COURSE_TERMS_STATUS_SUPERSEDED => Ok(Self::Superseded),
            other => Err(CourseCompletionTermsParseError {
                value: other.to_string(),
            }),
        }
    }

    pub fn can_transition_to(self, target: Self) -> bool {
        use CourseCompletionTermsStatus as Status;

        if self == target {
            return true;
        }

        match self {
            Status::Submitted | Status::Countered => matches!(
                target,
                Status::Countered
                    | Status::Accepted
                    | Status::Active
                    | Status::Rejected
                    | Status::Withdrawn
                    | Status::Superseded
            ),
            Status::Accepted => matches!(target, Status::Active | Status::Superseded),
            Status::Active => target == Status::Superseded,
            Status::Rejected | Status::Withdrawn | Status::Superseded => false,
        }
    }
}

impl CourseCompletionTermsAuditEventType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Proposed => COURSE_TERMS_EVENT_PROPOSED,
            Self::Countered => COURSE_TERMS_EVENT_COUNTERED,
            Self::Accepted => COURSE_TERMS_EVENT_ACCEPTED,
            Self::Activated => COURSE_TERMS_EVENT_ACTIVATED,
            Self::Rejected => COURSE_TERMS_EVENT_REJECTED,
            Self::Withdrawn => COURSE_TERMS_EVENT_WITHDRAWN,
            Self::Superseded => COURSE_TERMS_EVENT_SUPERSEDED,
        }
    }

    pub fn parse(value: &str) -> Result<Self, CourseCompletionTermsParseError> {
        match value {
            COURSE_TERMS_EVENT_PROPOSED => Ok(Self::Proposed),
            COURSE_TERMS_EVENT_COUNTERED => Ok(Self::Countered),
            COURSE_TERMS_EVENT_ACCEPTED => Ok(Self::Accepted),
            COURSE_TERMS_EVENT_ACTIVATED => Ok(Self::Activated),
            COURSE_TERMS_EVENT_REJECTED => Ok(Self::Rejected),
            COURSE_TERMS_EVENT_WITHDRAWN => Ok(Self::Withdrawn),
            COURSE_TERMS_EVENT_SUPERSEDED => Ok(Self::Superseded),
            other => Err(CourseCompletionTermsParseError {
                value: other.to_string(),
            }),
        }
    }
}

impl fmt::Display for CourseCompletionTermsStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for CourseCompletionTermsAuditEventType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for CourseCompletionTermsParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "unknown course completion terms '{}'",
            self.value
        )
    }
}
