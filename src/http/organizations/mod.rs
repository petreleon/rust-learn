mod course_dto;
mod course_nested_dto;
mod courses;
mod dashboard;
mod dto;
mod handlers;
mod member_audit;
mod member_invites;
mod member_list;
mod member_removal;
mod member_roles;
mod routes;
mod teacher_applications;

pub use routes::{configure_routes, organization_scope};
