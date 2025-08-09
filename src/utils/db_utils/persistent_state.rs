// src/utils/db_utils/persistent_state.rs
use diesel::prelude::*;
use diesel::{upsert::excluded, PgConnection, QueryResult};

use crate::db::schema::persistent_states;

/// Upsert a key/value into persistent_states
pub fn set_persistent_state(conn: &mut PgConnection, key_str: &str, value_str: &str) -> QueryResult<usize> {
    diesel::insert_into(persistent_states::table)
        .values((
            persistent_states::key.eq(key_str),
            persistent_states::value.eq(value_str),
        ))
        .on_conflict(persistent_states::key)
        .do_update()
        .set(persistent_states::value.eq(excluded(persistent_states::value)))
        .execute(conn)
}

/// Get a value from persistent_states by key
pub fn get_persistent_state(conn: &mut PgConnection, key_str: &str) -> QueryResult<Option<String>> {
    use crate::db::schema::persistent_states::dsl as ps;
    ps::persistent_states
        .select(ps::value)
        .filter(ps::key.eq(key_str))
        .first::<String>(conn)
        .optional()
}
