use std::fmt;

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
            Self::AssessmentCompletion => "assessment_completion",
            Self::CourseCompletion => "course_completion",
            Self::ManualCompletion => "manual_completion",
            Self::AdministrativeAdjustment => "administrative_adjustment",
        }
    }

    pub fn parse(value: &str) -> Result<Self, EventTypeParseError> {
        match value {
            "assessment_completion" => Ok(Self::AssessmentCompletion),
            "course_completion" => Ok(Self::CourseCompletion),
            "manual_completion" => Ok(Self::ManualCompletion),
            "administrative_adjustment" => Ok(Self::AdministrativeAdjustment),
            other => Err(EventTypeParseError {
                value: other.to_string(),
            }),
        }
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
    use super::RewardEventType;

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
}
