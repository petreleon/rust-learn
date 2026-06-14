#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PlatformRewardCandidatesQuery {
    pub status: Option<String>,
    pub search: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
