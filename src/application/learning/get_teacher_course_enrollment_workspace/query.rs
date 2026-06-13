const DEFAULT_COURSE_LIMIT: i64 = 25;
const MAX_COURSE_LIMIT: i64 = 100;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherCourseEnrollmentWorkspaceQuery {
    pub actor_user_id: i32,
    pub course_id: i32,
    pub status: Option<String>,
    pub limit: i64,
    pub offset: i64,
}

impl TeacherCourseEnrollmentWorkspaceQuery {
    pub fn new(
        actor_user_id: i32,
        course_id: i32,
        status: Option<String>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Self {
        Self {
            actor_user_id,
            course_id,
            status: normalize_optional_string(status)
                .map(|value| value.to_ascii_lowercase())
                .or_else(|| Some("open".to_string())),
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
    use super::TeacherCourseEnrollmentWorkspaceQuery;

    #[test]
    fn teacher_course_enrollment_query_normalizes_defaults() {
        let query = TeacherCourseEnrollmentWorkspaceQuery::new(
            7,
            9,
            Some("  PENDING ".to_string()),
            Some(500),
            Some(-10),
        );

        assert_eq!(query.actor_user_id, 7);
        assert_eq!(query.course_id, 9);
        assert_eq!(query.status.as_deref(), Some("pending"));
        assert_eq!(query.limit, 100);
        assert_eq!(query.offset, 0);
    }
}
