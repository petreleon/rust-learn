use super::{create_chapter, delete_chapter, list_chapters, update_chapter};
use crate::config::constants::permissions::Permissions;
use crate::middlewares::course_permission_middleware::CoursePermissionMiddleware;
use crate::models::param_type::ParamType;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/{id}/chapters")
            .route(
                web::get()
                    .to(list_chapters)
                    .wrap(CoursePermissionMiddleware::require(
                        Permissions::VIEW_COURSE.to_string(),
                        ParamType::Path,
                        "id".to_string(),
                    )),
            )
            .route(
                web::post()
                    .to(create_chapter)
                    .wrap(CoursePermissionMiddleware::require(
                        Permissions::MANAGE_COURSE_SETTINGS.to_string(),
                        ParamType::Path,
                        "id".to_string(),
                    )),
            ),
    )
    .service(
        web::resource("/{course_id}/chapters/{id}")
            .route(
                web::put()
                    .to(update_chapter)
                    .wrap(CoursePermissionMiddleware::require(
                        Permissions::MANAGE_COURSE_SETTINGS.to_string(),
                        ParamType::Path,
                        "course_id".to_string(),
                    )),
            )
            .route(
                web::delete()
                    .to(delete_chapter)
                    .wrap(CoursePermissionMiddleware::require(
                        Permissions::MANAGE_COURSE_SETTINGS.to_string(),
                        ParamType::Path,
                        "course_id".to_string(),
                    )),
            ),
    );
}
