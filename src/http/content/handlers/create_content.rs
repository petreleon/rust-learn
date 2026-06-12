use actix_web::{web, HttpResponse, Responder};

use crate::application::content::manage_content_item::{self, ContentItemError};
use crate::db::DbPool;
use crate::http::content::dto::{ContentItemResponse, CreateContentItemRequest};
use crate::infra::postgres::content::content_item_store::PostgresContentItemStore;
use crate::utils::notifications::NotificationsState;

use super::content_item_error_log;

// #[get("/chapters/{id}/contents")]
pub(in crate::http::content) async fn list_contents(
    path: web::Path<(i32, i32)>, // course_id, chapter_id
    pool: web::Data<DbPool>,
) -> impl Responder {
    let (course_id, chapter_id) = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };
    let mut store = PostgresContentItemStore::new(&mut conn);

    match manage_content_item::list_content_items(&mut store, course_id, chapter_id).await {
        Ok(list) => HttpResponse::Ok().json(
            list.into_iter()
                .map(ContentItemResponse::from)
                .collect::<Vec<_>>(),
        ),
        Err(ContentItemError::ChapterNotFound) => {
            HttpResponse::NotFound().body("Chapter not found")
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
    pool: web::Data<DbPool>,
    notifications: Option<web::Data<NotificationsState>>,
    req: web::Json<CreateContentItemRequest>,
) -> impl Responder {
    let (course_id, chapter_id) = path.into_inner();
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };
    let mut store = PostgresContentItemStore::new(&mut conn);
    let command = req.into_inner().into_command(chapter_id);

    match manage_content_item::create_content_item(&mut store, course_id, command).await {
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
                    if let Err(err) = notifications
                        .send_content_published_notification(
                            recipient_id,
                            course_id,
                            content.id,
                            &content.content_type,
                        )
                        .await
                    {
                        log::warn!(
                            "event=notification_send_failed kind=content_published course_id={} content_id={} target_user_id={} error={:?}",
                            course_id,
                            content.id,
                            recipient_id,
                            err
                        );
                    }
                }
            }

            HttpResponse::Created().json(ContentItemResponse::from(content))
        }
        Err(ContentItemError::ChapterNotFound) => {
            HttpResponse::NotFound().body("Chapter not found")
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
