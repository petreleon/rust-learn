include!("organizations/imports.rs");
include!("organizations/get_organization_courses.rs");
include!("organizations/get_organization_teacher_applications.rs");
include!("organizations/add_member_by_email_route.rs");
include!("organizations/organization_scope.rs");

pub fn configure_routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(organization_scope());
}
