use diesel::prelude::*;
use crate::db::schema::db_version_control;

#[derive(Queryable, Identifiable)]
#[diesel(table_name = db_version_control)]
pub struct DbVersionControl {
    pub id: i32,
    pub version: i32,
}

impl DbVersionControl {
    pub fn get_current_version(conn: &mut PgConnection) -> QueryResult<i32> {
        use diesel::dsl::max;
        
        let current_version_opt: Option<i32> = db_version_control::table
            .select(max(db_version_control::version))
            .first(conn)?;
            
        Ok(current_version_opt.unwrap_or(0))
    }

    pub fn update_version(conn: &mut PgConnection, new_version: i32) -> QueryResult<usize> {
        diesel::update(db_version_control::table.filter(db_version_control::id.eq(1)))
            .set(db_version_control::version.eq(new_version))
            .execute(conn)
    }
}
