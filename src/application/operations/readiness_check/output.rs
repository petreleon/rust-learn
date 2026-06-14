#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadinessOutput {
    pub status: ReadinessStatus,
    pub checks: Vec<DependencyCheckOutput>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadinessStatus {
    Ready,
    NotReady,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyCheckOutput {
    pub name: &'static str,
    pub status: DependencyStatus,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencyStatus {
    Ok,
    Failed,
}
