use actix_web::web;

use crate::application::content::manage_content_item::ContentItemError;
use crate::config::constants::permissions::Permissions;
use crate::middlewares::course_permission_middleware::CoursePermissionMiddleware;
use crate::models::param_type::ParamType;

mod create_content;
mod get_media_url;
mod get_upload_url;
mod process_content;

fn content_item_error_log(error: &ContentItemError) -> String {
    match error {
        ContentItemError::ChapterNotFound => "chapter_not_found".to_string(),
        ContentItemError::ContentNotFound => "content_not_found".to_string(),
        ContentItemError::Database(message) => message.clone(),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/{course_id}/chapters/{chapter_id}/contents")
            .route(web::get().to(create_content::list_contents).wrap(
                CoursePermissionMiddleware::require(
                    Permissions::VIEW_CONTENT.to_string(),
                    ParamType::Path,
                    "course_id".to_string(),
                ),
            ))
            .route(web::post().to(create_content::create_content).wrap(
                CoursePermissionMiddleware::require(
                    Permissions::CREATE_CONTENT.to_string(),
                    ParamType::Path,
                    "course_id".to_string(),
                ),
            )),
    )
    .service(
        web::resource("/{course_id}/chapters/{chapter_id}/contents/upload_url").route(
            web::post().to(get_upload_url::get_upload_url).wrap(
                CoursePermissionMiddleware::require(
                    Permissions::CREATE_CONTENT.to_string(),
                    ParamType::Path,
                    "course_id".to_string(),
                ),
            ),
        ),
    )
    .service(
        web::resource("/{course_id}/chapters/{chapter_id}/contents/{id}")
            .route(web::put().to(get_upload_url::update_content).wrap(
                CoursePermissionMiddleware::require(
                    Permissions::MODIFY_CONTENT.to_string(),
                    ParamType::Path,
                    "course_id".to_string(),
                ),
            ))
            .route(web::delete().to(get_upload_url::delete_content).wrap(
                CoursePermissionMiddleware::require(
                    Permissions::DELETE_CONTENT.to_string(),
                    ParamType::Path,
                    "course_id".to_string(),
                ),
            )),
    )
    .service(
        web::resource("/{course_id}/chapters/{chapter_id}/contents/{id}/process").route(
            web::post().to(process_content::process_content).wrap(
                CoursePermissionMiddleware::require(
                    Permissions::MODIFY_CONTENT.to_string(),
                    ParamType::Path,
                    "course_id".to_string(),
                ),
            ),
        ),
    )
    .service(
        web::resource("/{course_id}/chapters/{chapter_id}/contents/{id}/media").route(
            web::get()
                .to(get_media_url::get_media_url)
                .wrap(CoursePermissionMiddleware::require(
                    Permissions::VIEW_CONTENT.to_string(),
                    ParamType::Path,
                    "course_id".to_string(),
                )),
        ),
    );
}
