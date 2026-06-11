use super::*;

// ── Query constructors ──

#[test]
fn course_discovery_query_defaults() {
    let q = CourseDiscoveryQuery::new(None, None, None, None);
    assert_eq!(q.search, None);
    assert_eq!(q.organization_id, None);
    assert_eq!(q.limit, 25);
    assert_eq!(q.offset, 0);
}

#[test]
fn course_discovery_query_clamps_limits() {
    let q = CourseDiscoveryQuery::new(None, None, Some(0), Some(-1));
    assert_eq!(q.limit, 1);
    assert_eq!(q.offset, 0);
    let q2 = CourseDiscoveryQuery::new(None, None, Some(500), None);
    assert_eq!(q2.limit, 100);
}

#[test]
fn course_discovery_query_trims_search() {
    let q = CourseDiscoveryQuery::new(Some("  query  ".into()), None, None, None);
    assert_eq!(q.search, Some("query".into()));
    let q2 = CourseDiscoveryQuery::new(Some("   ".into()), None, None, None);
    assert_eq!(q2.search, None);
}

#[test]
fn learner_catalog_query_defaults() {
    let q = LearnerCourseCatalogQuery::new(None, None, None, None, None, None, None);
    assert_eq!(q.limit, 25);
    assert_eq!(q.offset, 0);
    assert_eq!(q.search, None);
    assert_eq!(q.reward_available, None);
}

#[test]
fn teacher_dashboard_query_defaults() {
    let q = TeacherCourseDashboardQuery::new(None, None, None, None);
    assert_eq!(q.limit, 25);
    assert_eq!(q.offset, 0);
}

#[test]
fn teacher_enrollment_query_defaults_status_to_open() {
    let q = TeacherCourseEnrollmentQuery::new(None, None, None);
    assert_eq!(q.status, Some("open".into()));
    assert_eq!(q.limit, 25);
    assert_eq!(q.offset, 0);
}

#[test]
fn teacher_enrollment_query_normalizes_custom_status() {
    let q = TeacherCourseEnrollmentQuery::new(Some("  PENDING  ".into()), None, None);
    assert_eq!(q.status, Some("pending".into()));
}

#[test]
fn organization_course_list_query_defaults() {
    let q = OrganizationCourseListQuery::new(None, None, None, None, None);
    assert_eq!(q.limit, 25);
    assert_eq!(q.offset, 0);
    assert_eq!(q.reward_available, None);
}

// ── CourseLifecycleError from diesel::Error ──

#[test]
fn course_lifecycle_diesel_not_found_maps_to_not_found() {
    assert_eq!(
        CourseLifecycleError::from(diesel::result::Error::NotFound),
        CourseLifecycleError::NotFound
    );
}

#[test]
fn course_lifecycle_diesel_other_errors_map_to_database() {
    assert!(matches!(
        CourseLifecycleError::from(diesel::result::Error::RollbackTransaction),
        CourseLifecycleError::Database(_)
    ));
}
