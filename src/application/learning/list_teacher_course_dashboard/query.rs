const DEFAULT_COURSE_LIMIT: i64 = 25;
const MAX_COURSE_LIMIT: i64 = 100;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherCourseDashboardListQuery {
    pub actor_user_id: i32,
    pub search: Option<String>,
    pub lifecycle_status: Option<String>,
    pub limit: i64,
    pub offset: i64,
}

impl TeacherCourseDashboardListQuery {
    pub fn new(
        actor_user_id: i32,
        search: Option<String>,
        lifecycle_status: Option<String>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Self {
        Self {
            actor_user_id,
            search: normalize_optional_string(search),
            lifecycle_status: normalize_optional_string(lifecycle_status),
            limit: limit
                .unwrap_or(DEFAULT_COURSE_LIMIT)
                .clamp(1, MAX_COURSE_LIMIT),
            offset: offset.unwrap_or(0).max(0),
        }
    }
}

fn normalize_optional_string(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

#[cfg(test)]
mod tests {
    use super::TeacherCourseDashboardListQuery;

    #[test]
    fn teacher_course_dashboard_query_normalizes_defaults() {
        let query = TeacherCourseDashboardListQuery::new(
            7,
            Some("  rust  ".to_string()),
            Some("  published ".to_string()),
            Some(500),
            Some(-10),
        );

        assert_eq!(query.actor_user_id, 7);
        assert_eq!(query.search.as_deref(), Some("rust"));
        assert_eq!(query.lifecycle_status.as_deref(), Some("published"));
        assert_eq!(query.limit, 100);
        assert_eq!(query.offset, 0);
    }
}
