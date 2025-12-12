use diesel::prelude::*;
use crate::db::schema::persistent_states;
use diesel::upsert::excluded;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

#[derive(Queryable, Insertable)]
#[diesel(table_name = persistent_states)]
pub struct PersistentState {
    pub id: i32,
    pub key: String,
    pub value: String,
}

impl PersistentState {
    pub async fn set(key: &str, value: &str, conn: &mut AsyncPgConnection) -> QueryResult<usize> {
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

    pub async fn get(key: &str, conn: &mut AsyncPgConnection) -> QueryResult<Option<String>> {
        persistent_states::table
            .select(persistent_states::value)
            .filter(persistent_states::key.eq(key))
            .first(conn)
            .await
            .optional()
    }
}
