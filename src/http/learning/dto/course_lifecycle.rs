use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct CourseLifecycleUpdateRequest {
    pub status: String,
}
