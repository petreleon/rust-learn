#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssessmentSubmissionError {
    NotFound,
    MaximumAttemptsReached,
    Connection(String),
    LoadFailed(String),
    SaveFailed(String),
}
