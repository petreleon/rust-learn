use actix_web::web;

use crate::config::constants::permissions::Permissions;
use crate::http::request_params::ParamType;
use crate::middlewares::course_permission_middleware::CoursePermissionMiddleware;

use super::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    configure_chapter_routes(cfg);
    configure_content_item_routes(cfg);
}

fn configure_chapter_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/{id}/chapters")
            .route(web::get().to(handlers::list_chapters).wrap(
                CoursePermissionMiddleware::require(
                    Permissions::VIEW_COURSE.to_string(),
                    ParamType::Path,
                    "id".to_string(),
                ),
            ))
            .route(web::post().to(handlers::create_chapter).wrap(
                CoursePermissionMiddleware::require(
                    Permissions::MANAGE_COURSE_SETTINGS.to_string(),
                    ParamType::Path,
                    "id".to_string(),
                ),
            )),
    )
    .service(
        web::resource("/{course_id}/chapters/{id}")
            .route(web::put().to(handlers::update_chapter).wrap(
                CoursePermissionMiddleware::require(
                    Permissions::MANAGE_COURSE_SETTINGS.to_string(),
                    ParamType::Path,
                    "course_id".to_string(),
                ),
            ))
            .route(web::delete().to(handlers::delete_chapter).wrap(
                CoursePermissionMiddleware::require(
                    Permissions::MANAGE_COURSE_SETTINGS.to_string(),
                    ParamType::Path,
                    "course_id".to_string(),
                ),
            )),
    );
}

fn configure_content_item_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/{course_id}/chapters/{chapter_id}/contents")
            .route(web::get().to(handlers::list_contents).wrap(
                CoursePermissionMiddleware::require(
                    Permissions::VIEW_CONTENT.to_string(),
                    ParamType::Path,
                    "course_id".to_string(),
                ),
            ))
            .route(web::post().to(handlers::create_content).wrap(
                CoursePermissionMiddleware::require(
                    Permissions::CREATE_CONTENT.to_string(),
                    ParamType::Path,
                    "course_id".to_string(),
                ),
            )),
    )
    .service(
        web::resource("/{course_id}/chapters/{chapter_id}/contents/upload_url").route(
            web::post()
                .to(handlers::get_upload_url)
                .wrap(CoursePermissionMiddleware::require(
                    Permissions::CREATE_CONTENT.to_string(),
                    ParamType::Path,
                    "course_id".to_string(),
                )),
        ),
    )
    .service(
        web::resource("/{course_id}/chapters/{chapter_id}/contents/{id}")
            .route(web::put().to(handlers::update_content).wrap(
                CoursePermissionMiddleware::require(
                    Permissions::MODIFY_CONTENT.to_string(),
                    ParamType::Path,
                    "course_id".to_string(),
                ),
            ))
            .route(web::delete().to(handlers::delete_content).wrap(
                CoursePermissionMiddleware::require(
                    Permissions::DELETE_CONTENT.to_string(),
                    ParamType::Path,
                    "course_id".to_string(),
                ),
            )),
    )
    .service(
        web::resource("/{course_id}/chapters/{chapter_id}/contents/{id}/process").route(
            web::post()
                .to(handlers::process_content)
                .wrap(CoursePermissionMiddleware::require(
                    Permissions::MODIFY_CONTENT.to_string(),
                    ParamType::Path,
                    "course_id".to_string(),
                )),
        ),
    )
    .service(
        web::resource("/{course_id}/chapters/{chapter_id}/contents/{id}/media").route(
            web::get()
                .to(handlers::get_media_url)
                .wrap(CoursePermissionMiddleware::require(
                    Permissions::VIEW_CONTENT.to_string(),
                    ParamType::Path,
                    "course_id".to_string(),
                )),
        ),
    );
}
