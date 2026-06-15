use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::content::manage_content_item::{ContentItemError, ContentItemOutput};
use crate::db::schema::{contents, user_role_course};
use crate::infra::postgres::content::mappers::content_item_output_from_record;
use crate::infra::postgres::models::content::{Content, NewContent, UpdateContent};

pub(super) async fn list_content_items(
    conn: &mut AsyncPgConnection,
    chapter_id: i32,
) -> Result<Vec<ContentItemOutput>, ContentItemError> {
    contents::table
        .filter(contents::chapter_id.eq(chapter_id))
        .order(contents::order.asc())
        .load::<Content>(conn)
        .await
        .map(|items| {
            items
                .into_iter()
                .map(content_item_output_from_record)
                .collect()
        })
        .map_err(map_database_error)
}

pub(super) async fn create_content_item(
    conn: &mut AsyncPgConnection,
    new_content: NewContent,
) -> Result<ContentItemOutput, ContentItemError> {
    diesel::insert_into(contents::table)
        .values(&new_content)
        .get_result::<Content>(conn)
        .await
        .map(content_item_output_from_record)
        .map_err(map_database_error)
}

pub(super) async fn list_course_content_recipients(
    conn: &mut AsyncPgConnection,
    course_id: i32,
) -> Result<Vec<i32>, ContentItemError> {
    user_role_course::table
        .filter(user_role_course::course_id.eq(course_id))
        .select(user_role_course::user_id)
        .distinct()
        .load::<Option<i32>>(conn)
        .await
        .map(|ids| ids.into_iter().flatten().collect())
        .map_err(map_database_error)
}

pub(super) async fn update_content_item(
    conn: &mut AsyncPgConnection,
    content_id: i32,
    update: UpdateContent,
) -> Result<ContentItemOutput, ContentItemError> {
    diesel::update(contents::table.find(content_id))
        .set(&update)
        .get_result::<Content>(conn)
        .await
        .map(content_item_output_from_record)
        .map_err(map_content_error)
}

pub(super) async fn delete_content_item(
    conn: &mut AsyncPgConnection,
    content_id: i32,
) -> Result<bool, ContentItemError> {
    diesel::delete(contents::table.find(content_id))
        .execute(conn)
        .await
        .map(|count| count > 0)
        .map_err(map_database_error)
}

fn map_content_error(error: diesel::result::Error) -> ContentItemError {
    match error {
        diesel::result::Error::NotFound => ContentItemError::ContentNotFound,
        other => ContentItemError::Database(other.to_string()),
    }
}

fn map_database_error(error: diesel::result::Error) -> ContentItemError {
    ContentItemError::Database(error.to_string())
}
