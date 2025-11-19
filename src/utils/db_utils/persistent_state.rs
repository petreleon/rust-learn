// src/utils/db_utils/persistent_state.rs
use diesel::prelude::*;
use diesel::{upsert::excluded, PgConnection, QueryResult};

use crate::models::persistent_state::PersistentState;

/// Upsert a key/value into persistent_states
pub fn set_persistent_state(conn: &mut PgConnection, key_str: &str, value_str: &str) -> QueryResult<usize> {
    PersistentState::set(key_str, value_str, conn)
}

/// Get a value from persistent_states by key
pub fn get_persistent_state(conn: &mut PgConnection, key_str: &str) -> QueryResult<Option<String>> {
    PersistentState::get(key_str, conn)
}
