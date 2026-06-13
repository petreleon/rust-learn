const DEFAULT_COURSE_LIMIT: i64 = 25;
const MAX_COURSE_LIMIT: i64 = 100;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnerCourseCatalogQuery {
    pub actor_user_id: i32,
    pub search: Option<String>,
    pub organization_id: Option<i32>,
    pub lifecycle_status: Option<String>,
    pub enrollment_status: Option<String>,
    pub reward_available: Option<bool>,
    pub limit: i64,
    pub offset: i64,
}

impl LearnerCourseCatalogQuery {
    pub fn new(
        actor_user_id: i32,
        search: Option<String>,
        organization_id: Option<i32>,
        lifecycle_status: Option<String>,
        enrollment_status: Option<String>,
        reward_available: Option<bool>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Self {
        Self {
            actor_user_id,
            search: normalize_optional_string(search),
            organization_id,
            lifecycle_status: normalize_optional_string(lifecycle_status),
            enrollment_status: normalize_optional_string(enrollment_status),
            reward_available,
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
    use super::LearnerCourseCatalogQuery;

    #[test]
    fn learner_course_catalog_query_normalizes_defaults() {
        let query = LearnerCourseCatalogQuery::new(
            7,
            Some("  rust  ".to_string()),
            Some(3),
            Some("  published ".to_string()),
            Some(" ".to_string()),
            Some(true),
            Some(500),
            Some(-10),
        );

        assert_eq!(query.actor_user_id, 7);
        assert_eq!(query.search.as_deref(), Some("rust"));
        assert_eq!(query.organization_id, Some(3));
        assert_eq!(query.lifecycle_status.as_deref(), Some("published"));
        assert_eq!(query.enrollment_status, None);
        assert_eq!(query.reward_available, Some(true));
        assert_eq!(query.limit, 100);
        assert_eq!(query.offset, 0);
    }
}
