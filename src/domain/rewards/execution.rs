use std::fmt;

pub const REWARD_EXECUTION_STATUS_QUEUED: &str = "queued";
pub const REWARD_EXECUTION_STATUS_FAILED: &str = "failed";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewardExecutionJobStatus {
    Queued,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionJobStatusParseError {
    value: String,
}

impl RewardExecutionJobStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Queued => REWARD_EXECUTION_STATUS_QUEUED,
            Self::Failed => REWARD_EXECUTION_STATUS_FAILED,
        }
    }

    pub fn parse(value: &str) -> Result<Self, ExecutionJobStatusParseError> {
        match value {
            REWARD_EXECUTION_STATUS_QUEUED => Ok(Self::Queued),
            REWARD_EXECUTION_STATUS_FAILED => Ok(Self::Failed),
            other => Err(ExecutionJobStatusParseError {
                value: other.to_string(),
            }),
        }
    }
}

impl fmt::Display for RewardExecutionJobStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Display for ExecutionJobStatusParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "unknown reward execution job status '{}'",
            self.value
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{RewardExecutionJobStatus, REWARD_EXECUTION_STATUS_FAILED};

    #[test]
    fn exposes_stable_execution_job_keys() {
        assert_eq!(RewardExecutionJobStatus::Queued.as_str(), "queued");
        assert_eq!(
            RewardExecutionJobStatus::Failed.as_str(),
            REWARD_EXECUTION_STATUS_FAILED
        );
    }

    #[test]
    fn parses_known_execution_job_status() {
        assert_eq!(
            RewardExecutionJobStatus::parse("failed").unwrap(),
            RewardExecutionJobStatus::Failed
        );
    }

    #[test]
    fn rejects_unknown_execution_job_status() {
        assert!(RewardExecutionJobStatus::parse("retrying").is_err());
    }
}
