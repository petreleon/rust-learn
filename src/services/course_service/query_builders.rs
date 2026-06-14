use super::query_types::{
    CourseDiscoveryQuery, LearnerCourseCatalogQuery, OrganizationCourseListQuery,
    TeacherCourseDashboardQuery, TeacherCourseEnrollmentQuery,
};
use super::shared_helpers::normalize_optional_string;
use super::{DEFAULT_COURSE_LIMIT, MAX_COURSE_LIMIT};

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

        CourseDiscoveryQuery {
            search,
            organization_id,
            limit,
            offset,
        }
    }
}

impl LearnerCourseCatalogQuery {
    pub fn new(
        search: Option<String>,
        organization_id: Option<i32>,
        lifecycle_status: Option<String>,
        enrollment_status: Option<String>,
        reward_available: Option<bool>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Self {
        let search = normalize_optional_string(search);
        let lifecycle_status = normalize_optional_string(lifecycle_status);
        let enrollment_status = normalize_optional_string(enrollment_status);
        let limit = limit
            .unwrap_or(DEFAULT_COURSE_LIMIT)
            .clamp(1, MAX_COURSE_LIMIT);
        let offset = offset.unwrap_or(0).max(0);

        LearnerCourseCatalogQuery {
            search,
            organization_id,
            lifecycle_status,
            enrollment_status,
            reward_available,
            limit,
            offset,
        }
    }
}

impl TeacherCourseDashboardQuery {
    pub fn new(
        search: Option<String>,
        lifecycle_status: Option<String>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Self {
        let search = normalize_optional_string(search);
        let lifecycle_status = normalize_optional_string(lifecycle_status);
        let limit = limit
            .unwrap_or(DEFAULT_COURSE_LIMIT)
            .clamp(1, MAX_COURSE_LIMIT);
        let offset = offset.unwrap_or(0).max(0);

        TeacherCourseDashboardQuery {
            search,
            lifecycle_status,
            limit,
            offset,
        }
    }
}

impl OrganizationCourseListQuery {
    pub fn new(
        search: Option<String>,
        lifecycle_status: Option<String>,
        reward_available: Option<bool>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Self {
        let search = normalize_optional_string(search);
        let lifecycle_status = normalize_optional_string(lifecycle_status);
        let limit = limit
            .unwrap_or(DEFAULT_COURSE_LIMIT)
            .clamp(1, MAX_COURSE_LIMIT);
        let offset = offset.unwrap_or(0).max(0);

        OrganizationCourseListQuery {
            search,
            lifecycle_status,
            reward_available,
            limit,
            offset,
        }
    }
}

impl TeacherCourseEnrollmentQuery {
    pub fn new(status: Option<String>, limit: Option<i64>, offset: Option<i64>) -> Self {
        let status = normalize_optional_string(status)
            .map(|value| value.to_ascii_lowercase())
            .or_else(|| Some("open".to_string()));
        let limit = limit
            .unwrap_or(DEFAULT_COURSE_LIMIT)
            .clamp(1, MAX_COURSE_LIMIT);
        let offset = offset.unwrap_or(0).max(0);

        TeacherCourseEnrollmentQuery {
            status,
            limit,
            offset,
        }
    }
}
