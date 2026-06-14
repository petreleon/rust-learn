use actix_web::web;

use crate::config::constants::permissions::Permissions;
use crate::http::request_params::ParamType;
use crate::middlewares::organization_permission_middleware::OrganizationPermissionMiddleware;
use crate::middlewares::platform_permission_middleware::PlatformPermissionMiddleware;

use super::{
    courses, dashboard, handlers, member_audit, member_invites, member_list, member_removal,
    member_roles, teacher_applications,
};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(organization_scope());
}

pub fn organization_scope() -> actix_web::Scope {
    web::scope("/organizations")
        .service(crate::http::rewards::organization_scope_reward_candidate_submission_resource())
        .service(
            web::resource("")
                .route(web::get().to(handlers::list_organizations).wrap(
                    PlatformPermissionMiddleware::require(
                        Permissions::VIEW_ORGANIZATION.to_string(),
                    ),
                ))
                .route(web::post().to(handlers::create_organization).wrap(
                    PlatformPermissionMiddleware::require(
                        Permissions::CREATE_ORGANIZATION.to_string(),
                    ),
                )),
        )
        .service(
            web::resource("/{id}/courses").route(web::get().to(courses::get_organization_courses)),
        )
        .service(
            web::resource("/{id}/members")
                .route(web::get().to(member_list::get_organization_members))
                .route(web::post().to(member_invites::add_member_by_email_route)),
        )
        .service(
            web::resource("/{id}/dashboard")
                .route(web::get().to(dashboard::get_organization_dashboard)),
        )
        .service(
            web::resource("/{id}/members/{user_id}/audit")
                .route(web::get().to(member_audit::get_member_audit_route)),
        )
        .service(
            web::resource("/{id}")
                .route(web::get().to(handlers::get_organization).wrap(
                    PlatformPermissionMiddleware::require(
                        Permissions::VIEW_ORGANIZATION.to_string(),
                    ),
                ))
                .route(web::put().to(handlers::update_organization).wrap(
                    OrganizationPermissionMiddleware::require(
                        Permissions::MANAGE_ORG_SETTINGS.to_string(),
                        ParamType::Path,
                        "id".to_string(),
                    ),
                ))
                .route(web::delete().to(handlers::delete_organization).wrap(
                    OrganizationPermissionMiddleware::require(
                        Permissions::MANAGE_ORG_SETTINGS.to_string(),
                        ParamType::Path,
                        "id".to_string(),
                    ),
                )),
        )
        .service(
            web::resource("/{id}/users/{user_id}/roles")
                .route(web::post().to(member_roles::assign_role)),
        )
        .service(
            web::resource("/{id}/users/{user_id}")
                .route(web::delete().to(member_removal::remove_organization_member_route)),
        )
        .service(
            web::resource("/{id}/teacher-applications")
                .route(web::get().to(teacher_applications::get_organization_teacher_applications))
                .route(
                    web::post()
                        .to(crate::http::teacher_applications::nominate_application)
                        .wrap(OrganizationPermissionMiddleware::require(
                            Permissions::NOMINATE_TEACHER_FOR_PLATFORM_REVIEW.to_string(),
                            ParamType::Path,
                            "id".to_string(),
                        )),
                ),
        )
}
