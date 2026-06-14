use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::content::manage_content_item::ContentItemError;
use crate::db::schema::{chapters, contents};

pub(super) async fn ensure_chapter_belongs_to_course(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    chapter_id: i32,
) -> Result<(), ContentItemError> {
    chapters::table
        .filter(chapters::id.eq(chapter_id))
        .filter(chapters::course_id.eq(course_id))
        .select(chapters::id)
        .first::<i32>(conn)
        .await
        .map(|_| ())
        .map_err(map_chapter_error)
}

pub(super) async fn ensure_content_belongs_to_chapter(
    conn: &mut AsyncPgConnection,
    chapter_id: i32,
    content_id: i32,
) -> Result<(), ContentItemError> {
    contents::table
        .filter(contents::id.eq(content_id))
        .filter(contents::chapter_id.eq(chapter_id))
        .select(contents::id)
        .first::<i32>(conn)
        .await
        .map(|_| ())
        .map_err(map_content_error)
}

fn map_chapter_error(error: diesel::result::Error) -> ContentItemError {
    match error {
        diesel::result::Error::NotFound => ContentItemError::ChapterNotFound,
        other => ContentItemError::Database(other.to_string()),
    }
}

fn map_content_error(error: diesel::result::Error) -> ContentItemError {
    match error {
        diesel::result::Error::NotFound => ContentItemError::ContentNotFound,
        other => ContentItemError::Database(other.to_string()),
    }
}
