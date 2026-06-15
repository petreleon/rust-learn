use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::infra::postgres::schema::db_version_control;

pub async fn get_current_version(conn: &mut AsyncPgConnection) -> QueryResult<i32> {
    let current_version_opt: Option<i32> = db_version_control::table
        .select(diesel::dsl::max(db_version_control::version))
        .first(conn)
        .await?;

    Ok(current_version_opt.unwrap_or(0))
}

pub async fn update_version(conn: &mut AsyncPgConnection, new_version: i32) -> QueryResult<usize> {
    diesel::insert_into(db_version_control::table)
        .values((
            db_version_control::id.eq(1),
            db_version_control::version.eq(new_version),
        ))
        .on_conflict(db_version_control::id)
        .do_update()
        .set(db_version_control::version.eq(new_version))
        .execute(conn)
        .await
}
