#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssessmentSubmissionError {
    NotFound,
    MaximumAttemptsReached,
    LoadFailed(String),
    SaveFailed(String),
}
