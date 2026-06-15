use std::sync::Arc;

use actix_web::{http::StatusCode, web};

use crate::application::content::manage_content_item::ContentItemUseCases;
use crate::application::notifications::delivery::{
    ContentPublishedNotification, NotificationDeliveryUseCase,
};
use crate::http::content::dto::{ContentItemResponse, CreateContentItemRequest};
use crate::http::content::errors::content_item_error;
use crate::http::errors::ApiError;

// #[get("/chapters/{id}/contents")]
pub(in crate::http::content) async fn list_contents(
    path: web::Path<(i32, i32)>, // course_id, chapter_id
    content_item_use_cases: web::Data<Arc<dyn ContentItemUseCases>>,
) -> Result<web::Json<Vec<ContentItemResponse>>, ApiError> {
    let (course_id, chapter_id) = path.into_inner();

    content_item_use_cases
        .list_content_items(course_id, chapter_id)
        .await
        .map(|items| items.into_iter().map(ContentItemResponse::from).collect())
        .map(web::Json)
        .map_err(|error| {
            content_item_error(
                "content_list_failed",
                format!("course_id={} chapter_id={}", course_id, chapter_id),
                "Failed to list contents",
                error,
            )
        })
}

pub(in crate::http::content) async fn create_content(
    path: web::Path<(i32, i32)>, // course_id, chapter_id
    content_item_use_cases: web::Data<Arc<dyn ContentItemUseCases>>,
    notifications: Option<web::Data<Arc<dyn NotificationDeliveryUseCase>>>,
    req: web::Json<CreateContentItemRequest>,
) -> Result<(web::Json<ContentItemResponse>, StatusCode), ApiError> {
    let (course_id, chapter_id) = path.into_inner();
    let command = req.into_inner().into_command(chapter_id);

    let output = content_item_use_cases
        .create_content_item(course_id, command)
        .await
        .map_err(|error| {
            content_item_error(
                "content_create_failed",
                format!("course_id={} chapter_id={}", course_id, chapter_id),
                "Failed to create content",
                error,
            )
        })?;

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
    Ok((
        web::Json(ContentItemResponse::from(content)),
        StatusCode::CREATED,
    ))
}
