use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::db::schema::db_version_control;

#[derive(Queryable, Identifiable)]
#[diesel(table_name = db_version_control)]
pub struct DbVersionControl {
    pub id: i32,
    pub version: i32,
}

impl DbVersionControl {
    pub async fn get_current_version(conn: &mut AsyncPgConnection) -> QueryResult<i32> {
        use diesel::dsl::max;
        
        let current_version_opt: Option<i32> = db_version_control::table
            .select(max(db_version_control::version))
            .first(conn)
            .await?;
            
        Ok(current_version_opt.unwrap_or(0))
    }

    pub async fn update_version(conn: &mut AsyncPgConnection, new_version: i32) -> QueryResult<usize> {
        diesel::update(db_version_control::table.filter(db_version_control::id.eq(1)))
            .set(db_version_control::version.eq(new_version))
            .execute(conn)
            .await
    }
}
