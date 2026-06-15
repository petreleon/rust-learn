use actix_web::web;

use crate::domain::access_control::permissions::Permissions;
use crate::http::middlewares::course_permission_middleware::CoursePermissionMiddleware;
use crate::http::middlewares::platform_permission_middleware::PlatformPermissionMiddleware;
use crate::http::request_params::ParamType;

use super::{
    assessments, catalog, enrollment, lifecycle, management, organizations, progress, roles,
    teaching,
};

pub fn course_scope() -> actix_web::Scope {
    web::scope("/courses")
        .configure(crate::http::content::configure_routes)
        .service(crate::http::rewards::course_scope_reward_candidate_submission_resource())
        .service(crate::http::rewards::course_scope_teacher_reward_candidate_decision_resource())
        .service(
            web::resource("/catalog").route(web::get().to(catalog::list_learner_course_catalog)),
        )
        .service(
            web::resource("/teaching")
                .route(web::get().to(teaching::list_teacher_course_dashboard)),
        )
        .service(
            web::resource("/teaching/{id}/enrollments")
                .route(web::get().to(teaching::get_teacher_course_enrollment_workspace_route)),
        )
        .service(
            web::resource("/teaching/{id}/students")
                .route(web::get().to(teaching::get_teacher_course_students_route)),
        )
        .service(
            web::resource("/teaching/{id}")
                .route(web::get().to(teaching::get_teacher_course_workspace_route)),
        )
        .service(
            web::resource("/catalog/{id}/learn")
                .route(web::get().to(catalog::get_learner_course_learning_route)),
        )
        .service(
            web::resource("/catalog/{id}")
                .route(web::get().to(catalog::get_learner_course_catalog_detail)),
        )
        .service(
            web::resource("")
                .route(web::get().to(catalog::list_courses).wrap(
                    PlatformPermissionMiddleware::require(Permissions::VIEW_COURSE.to_string()),
                ))
                .route(web::post().to(management::create_course)),
        )
        .service(web::resource("/{id}/organizations").route(
            web::get().to(organizations::get_course_organizations).wrap(
                PlatformPermissionMiddleware::require(Permissions::VIEW_COURSE.to_string()),
            ),
        ))
        .service(
            web::resource("/{id}/lifecycle")
                .route(web::put().to(lifecycle::update_course_lifecycle)),
        )
        .service(
            web::resource("/{id}/join-requests")
                .route(web::post().to(enrollment::request_course_join)),
        )
        .service(
            web::resource("/{id}/join-requests/{request_id}/decision")
                .route(web::put().to(enrollment::decide_course_join_request)),
        )
        .service(
            web::resource("/{id}/enrollments/{user_id}")
                .route(web::delete().to(enrollment::remove_course_enrollment)),
        )
        .service(
            web::resource("/{id}/progress")
                .route(web::get().to(progress::get_learner_progress_route))
                .route(web::post().to(progress::save_learner_progress_route)),
        )
        .service(
            web::resource("/{id}/assessments")
                .route(web::get().to(assessments::list_course_assessments)),
        )
        .service(
            web::resource("/{id}/assessments/{assessment_id}/submit")
                .route(web::post().to(assessments::submit_assessment_attempt)),
        )
        .service(
            web::resource("/{id}/assessments/{assessment_id}/attempts")
                .route(web::get().to(assessments::list_assessment_attempts)),
        )
        .service(
            web::resource("/{id}")
                .route(web::get().to(catalog::get_course).wrap(
                    PlatformPermissionMiddleware::require(Permissions::VIEW_COURSE.to_string()),
                ))
                .route(web::put().to(management::update_course))
                .route(web::delete().to(management::delete_course).wrap(
                    CoursePermissionMiddleware::require(
                        Permissions::DELETE_COURSE.to_string(),
                        ParamType::Path,
                        "id".to_string(),
                    ),
                )),
        )
        .service(
            web::resource("/{id}/users/{user_id}/roles").route(
                web::post()
                    .to(roles::assign_role)
                    .wrap(CoursePermissionMiddleware::require(
                        Permissions::MANAGE_COURSE_ENROLLMENTS.to_string(),
                        ParamType::Path,
                        "id".to_string(),
                    )),
            ),
        )
}
