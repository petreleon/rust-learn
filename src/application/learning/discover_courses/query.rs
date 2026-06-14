const DEFAULT_COURSE_LIMIT: i64 = 25;
const MAX_COURSE_LIMIT: i64 = 100;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseDiscoveryQuery {
    pub search: Option<String>,
    pub organization_id: Option<i32>,
    pub limit: i64,
    pub offset: i64,
}

impl CourseDiscoveryQuery {
    pub fn new(
        search: Option<String>,
        organization_id: Option<i32>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Self {
        let search = search
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty());
        let limit = limit
            .unwrap_or(DEFAULT_COURSE_LIMIT)
            .clamp(1, MAX_COURSE_LIMIT);
        let offset = offset.unwrap_or(0).max(0);

        Self {
            search,
            organization_id,
            limit,
            offset,
        }
    }
}
