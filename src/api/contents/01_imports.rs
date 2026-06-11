use crate::config::constants::permissions::Permissions;
use crate::db::schema::chapters;
use crate::db::schema::contents;
use crate::db::schema::upload_jobs;
use crate::db::schema::user_role_course;
use crate::db::DbPool;
use crate::middlewares::course_permission_middleware::CoursePermissionMiddleware;
use crate::models::content::{Content, NewContent, UpdateContent};
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

async fn ensure_content_belongs_to_chapter(
    conn: &mut AsyncPgConnection,
    chapter_id: i32,
    content_id: i32,
) -> Result<(), HttpResponse> {
    match contents::table
        .filter(contents::id.eq(content_id))
        .filter(contents::chapter_id.eq(chapter_id))
        .select(contents::id)
        .first::<i32>(conn)
        .await
    {
        Ok(_) => Ok(()),
        Err(diesel::result::Error::NotFound) => {
            Err(HttpResponse::NotFound().body("Content not found"))
        }
        Err(e) => {
            log::error!(
                "event=content_scope_check_failed chapter_id={} content_id={} error={}",
                chapter_id,
                content_id,
                e
            );
            Err(HttpResponse::InternalServerError().body("Failed to fetch content"))
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
    if let Err(response) = ensure_chapter_belongs_to_course(&mut conn, course_id, chapter_id).await
    {
        return response;
    }

    let result = contents::table
        .filter(contents::chapter_id.eq(chapter_id))
        .order(contents::order.asc())
        .load::<Content>(&mut conn)
        .await;

    match result {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e) => {
            log::error!(
                "event=content_list_failed chapter_id={} error={}",
                chapter_id,
                e
            );
            HttpResponse::InternalServerError().body("Failed to list contents")
        }
    }
}

#[derive(serde::Deserialize)]
pub struct CreateContentRequest {
    pub order: i32,
    pub content_type: String,
    pub data: Option<String>,
}
