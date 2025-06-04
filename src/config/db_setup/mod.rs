// src/config/db_setup/mod.rs

pub mod updates;

use diesel::{dsl::max, prelude::*};
use diesel::pg::PgConnection;
use crate::db::schema::db_version_control::dsl::*;
use self::updates::{apply_update_v1, apply_update_v2};
fn updates() -> Vec<(i32, for<'a> fn(&'a mut diesel::PgConnection) -> Result<(), diesel::result::Error>)> {
    vec![
        (1, apply_update_v1),
        (2, apply_update_v2)
    ]
}

pub fn version_updater(conn: &mut PgConnection) -> QueryResult<()> {

    let current_version_result = db_version_control
        .select(max(version))
        .first::<Option<i32>>(conn);

    let current_version = match current_version_result {
        Ok(Some(ver)) => ver,
        _ => 0,
    };

    let mut max_version = 0;

    for (target_version, update_function) in updates() {
        if current_version < target_version {
            update_function(conn)?;
        }
        if target_version > max_version {
            max_version = target_version;
        }
    }

    diesel::update(db_version_control.filter(id.eq(1)))
        .set(version.eq(max_version))
        .execute(conn)?;
    Ok(())
}
