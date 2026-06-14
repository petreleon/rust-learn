use diesel::prelude::*;
use diesel::upsert::excluded;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::persistent_states;

pub async fn set_persistent_state(
    conn: &mut AsyncPgConnection,
    key: &str,
    value: &str,
) -> QueryResult<usize> {
    diesel::insert_into(persistent_states::table)
        .values((
            persistent_states::key.eq(key),
            persistent_states::value.eq(value),
        ))
        .on_conflict(persistent_states::key)
        .do_update()
        .set(persistent_states::value.eq(excluded(persistent_states::value)))
        .execute(conn)
        .await
}

pub async fn get_persistent_state(
    conn: &mut AsyncPgConnection,
    key: &str,
) -> QueryResult<Option<String>> {
    persistent_states::table
        .select(persistent_states::value)
        .filter(persistent_states::key.eq(key))
        .first(conn)
        .await
        .optional()
}
