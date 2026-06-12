use crate::application::content::manage_content_item::{self, ContentItemError};
use crate::application::content::request_upload_url::{
    request_upload_url as request_upload_url_for_content, ContentUploadUrlError,
};
use crate::config::constants::permissions::Permissions;
use crate::db::schema::chapters;
use crate::db::schema::contents;
use crate::db::schema::upload_jobs;
use crate::db::DbPool;
use crate::http::content::dto::{
    ContentItemResponse, CreateContentItemRequest, RequestUploadUrlRequest, UpdateContentItemRequest,
    UploadUrlResponse,
};
use crate::infra::object_storage::content::upload_url_provider::S3ContentUploadUrlProvider;
use crate::infra::postgres::content::content_item_store::PostgresContentItemStore;
use crate::infra::postgres::content::upload_scope_store::PostgresContentUploadScopeStore;
use crate::middlewares::course_permission_middleware::CoursePermissionMiddleware;
use crate::models::content::Content;
use crate::models::param_type::ParamType;
use crate::models::upload_job::NewUploadJob;
use crate::utils::notifications::NotificationsState;
use crate::utils::request_auth::authenticated_user_id;
use crate::utils::s3_utils::S3State;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

async fn ensure_chapter_belongs_to_course(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    chapter_id: i32,
) -> Result<(), HttpResponse> {
    match chapters::table
        .filter(chapters::id.eq(chapter_id))
        .filter(chapters::course_id.eq(course_id))
        .select(chapters::id)
        .first::<i32>(conn)
        .await
    {
        Ok(_) => Ok(()),
        Err(diesel::result::Error::NotFound) => {
            Err(HttpResponse::NotFound().body("Chapter not found"))
        }
        Err(e) => {
            log::error!(
                "event=content_chapter_scope_check_failed course_id={} chapter_id={} error={}",
                course_id,
                chapter_id,
                e
            );
            Err(HttpResponse::InternalServerError().body("Failed to fetch chapter"))
        }
    }
}

// #[get("/chapters/{id}/contents")]
async fn list_contents(
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
        Err(ContentItemError::ChapterNotFound) => HttpResponse::NotFound().body("Chapter not found"),
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

fn content_item_error_log(error: &ContentItemError) -> String {
    match error {
        ContentItemError::ChapterNotFound => "chapter_not_found".to_string(),
        ContentItemError::ContentNotFound => "content_not_found".to_string(),
        ContentItemError::Database(message) => message.clone(),
    }
}
