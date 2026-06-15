use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::content::manage_content_item::{ContentItemError, ContentItemUseCases};
use crate::application::notifications::delivery::{
    ContentPublishedNotification, NotificationDeliveryUseCase,
};
use crate::http::content::dto::{ContentItemResponse, CreateContentItemRequest};

use super::content_item_error_log;

// #[get("/chapters/{id}/contents")]
pub(in crate::http::content) async fn list_contents(
    path: web::Path<(i32, i32)>, // course_id, chapter_id
    content_item_use_cases: web::Data<Arc<dyn ContentItemUseCases>>,
) -> impl Responder {
    let (course_id, chapter_id) = path.into_inner();

    match content_item_use_cases
        .list_content_items(course_id, chapter_id)
        .await
    {
        Ok(list) => HttpResponse::Ok().json(
            list.into_iter()
                .map(ContentItemResponse::from)
                .collect::<Vec<_>>(),
        ),
        Err(ContentItemError::ChapterNotFound) => {
            HttpResponse::NotFound().body("Chapter not found")
        }
        Err(ContentItemError::Connection(_)) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        Err(e) => {
            log::error!(
                "event=content_list_failed chapter_id={} error={}",
                chapter_id,
                content_item_error_log(&e)
            );
            HttpResponse::InternalServerError().body("Failed to list contents")
        }
    }
}

pub(in crate::http::content) async fn create_content(
    path: web::Path<(i32, i32)>, // course_id, chapter_id
    content_item_use_cases: web::Data<Arc<dyn ContentItemUseCases>>,
    notifications: Option<web::Data<Arc<dyn NotificationDeliveryUseCase>>>,
    req: web::Json<CreateContentItemRequest>,
) -> impl Responder {
    let (course_id, chapter_id) = path.into_inner();
    let command = req.into_inner().into_command(chapter_id);

    match content_item_use_cases
        .create_content_item(course_id, command)
        .await
    {
        Ok(output) => {
            let content = output.content;
            if let Some(notifications) = notifications {
                if let Some(err) = &output.notification_recipient_lookup_error {
                    log::warn!(
                        "event=notification_recipient_query_failed kind=content_published course_id={} content_id={} error={}",
                        course_id,
                        content.id,
                        err
                    );
                }

                for recipient_id in output.notification_recipient_ids {
                    let notification = ContentPublishedNotification {
                        recipient_user_id: recipient_id,
                        course_id,
                        content_id: content.id,
                        content_type: content.content_type.clone(),
                    };

                    if let Err(err) = notifications.send_content_published(notification).await {
                        log::warn!(
                            "event=notification_send_failed kind=content_published course_id={} content_id={} target_user_id={} error={}",
                            course_id,
                            content.id,
                            recipient_id,
                            err.message()
                        );
                    }
                }
            }

            HttpResponse::Created().json(ContentItemResponse::from(content))
        }
        Err(ContentItemError::ChapterNotFound) => {
            HttpResponse::NotFound().body("Chapter not found")
        }
        Err(ContentItemError::Connection(_)) => {
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        Err(e) => {
            log::error!(
                "event=content_create_failed course_id={} chapter_id={} error={}",
                course_id,
                chapter_id,
                content_item_error_log(&e)
            );
            HttpResponse::InternalServerError().body("Failed to create content")
        }
    }
}
