use std::fmt;

pub const REWARD_EVENT_ASSESSMENT_COMPLETION: &str = "assessment_completion";
pub const REWARD_EVENT_COURSE_COMPLETION: &str = "course_completion";
pub const REWARD_EVENT_MANUAL_COMPLETION: &str = "manual_completion";
pub const REWARD_EVENT_ADMINISTRATIVE_ADJUSTMENT: &str = "administrative_adjustment";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewardEventType {
    AssessmentCompletion,
    CourseCompletion,
    ManualCompletion,
    AdministrativeAdjustment,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventTypeParseError {
    value: String,
}

impl RewardEventType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AssessmentCompletion => REWARD_EVENT_ASSESSMENT_COMPLETION,
            Self::CourseCompletion => REWARD_EVENT_COURSE_COMPLETION,
            Self::ManualCompletion => REWARD_EVENT_MANUAL_COMPLETION,
            Self::AdministrativeAdjustment => REWARD_EVENT_ADMINISTRATIVE_ADJUSTMENT,
        }
    }

    pub fn parse(value: &str) -> Result<Self, EventTypeParseError> {
        match value {
            REWARD_EVENT_ASSESSMENT_COMPLETION => Ok(Self::AssessmentCompletion),
            REWARD_EVENT_COURSE_COMPLETION => Ok(Self::CourseCompletion),
            REWARD_EVENT_MANUAL_COMPLETION => Ok(Self::ManualCompletion),
            REWARD_EVENT_ADMINISTRATIVE_ADJUSTMENT => Ok(Self::AdministrativeAdjustment),
            other => Err(EventTypeParseError {
                value: other.to_string(),
            }),
        }
    }

    pub fn normalize(value: &str) -> Result<Self, EventTypeParseError> {
        let normalized = value.trim().to_ascii_lowercase().replace(['-', ' '], "_");
        Self::parse(&normalized)
    }
}

impl fmt::Display for RewardEventType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for EventTypeParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "unknown reward event type '{}'", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::{RewardEventType, REWARD_EVENT_COURSE_COMPLETION};

    #[test]
    fn exposes_stable_event_keys() {
        assert_eq!(
            RewardEventType::CourseCompletion.as_str(),
            REWARD_EVENT_COURSE_COMPLETION
        );
        assert_eq!(
            RewardEventType::AdministrativeAdjustment.as_str(),
            "administrative_adjustment"
        );
    }

    #[test]
    fn parses_known_event_types() {
        assert_eq!(
            RewardEventType::parse("course_completion").unwrap(),
            RewardEventType::CourseCompletion
        );
    }

    #[test]
    fn rejects_unknown_event_type() {
        assert!(RewardEventType::parse("course_started").is_err());
    }

    #[test]
    fn normalizes_common_event_type_spellings() {
        assert_eq!(
            RewardEventType::normalize(" course-completion ").unwrap(),
            RewardEventType::CourseCompletion
        );
        assert_eq!(
            RewardEventType::normalize("assessment completion").unwrap(),
            RewardEventType::AssessmentCompletion
        );
    }
}
