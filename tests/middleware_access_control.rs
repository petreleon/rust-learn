include!("middleware_access_control/01_imports.rs");
include!("middleware_access_control/02_test_platform_permission_middleware.rs");
include!("middleware_access_control/03_test_organization_permission_middleware.rs");
include!("middleware_access_control/04_test_course_permission_middleware.rs");
include!("middleware_access_control/05_course_read_routes_require_view_course_permission.rs");
include!(
    "middleware_access_control/06_role_read_routes_require_view_role_assignments_permission.rs"
);
