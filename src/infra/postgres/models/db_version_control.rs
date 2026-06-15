use crate::infra::postgres::schema::db_version_control;
use diesel::prelude::*;

#[derive(Queryable, Identifiable)]
#[diesel(table_name = db_version_control)]
pub struct DbVersionControl {
    pub id: i32,
    pub version: i32,
}
