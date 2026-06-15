use crate::db::schema::persistent_states;
use diesel::prelude::*;

#[derive(Queryable, Insertable)]
#[diesel(table_name = persistent_states)]
pub struct PersistentState {
    pub id: i32,
    pub key: String,
    pub value: String,
}
