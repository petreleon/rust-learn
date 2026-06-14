#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseDiscoveryCourseOutput {
    pub id: i32,
    pub title: String,
    pub lifecycle_status: String,
    pub description: Option<String>,
    pub topics: Option<String>,
    pub prerequisites: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseDiscoveryOutput {
    pub courses: Vec<CourseDiscoveryCourseOutput>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub search: Option<String>,
    pub organization_id: Option<i32>,
}
