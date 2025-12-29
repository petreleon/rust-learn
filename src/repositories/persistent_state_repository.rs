use diesel::prelude::*;
use diesel::QueryResult;
use diesel_async::AsyncPgConnection;

use crate::models::persistent_state::PersistentState;

/// Upsert a key/value into persistent_states
pub async fn set_persistent_state(conn: &mut AsyncPgConnection, key_str: &str, value_str: &str) -> QueryResult<usize> {
    PersistentState::set(key_str, value_str, conn).await
}

/// Get a value from persistent_states by key
pub async fn get_persistent_state(conn: &mut AsyncPgConnection, key_str: &str) -> QueryResult<Option<String>> {
    PersistentState::get(key_str, conn).await
}
