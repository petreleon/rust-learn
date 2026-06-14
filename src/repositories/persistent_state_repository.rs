use diesel::QueryResult;
use diesel_async::AsyncPgConnection;

use crate::infra::postgres::operations::persistent_state as persistent_state_records;

/// Upsert a key/value into persistent_states
pub async fn set_persistent_state(
    conn: &mut AsyncPgConnection,
    key_str: &str,
    value_str: &str,
) -> QueryResult<usize> {
    persistent_state_records::set_persistent_state(conn, key_str, value_str).await
}

/// Get a value from persistent_states by key
pub async fn get_persistent_state(
    conn: &mut AsyncPgConnection,
    key_str: &str,
) -> QueryResult<Option<String>> {
    persistent_state_records::get_persistent_state(conn, key_str).await
}
