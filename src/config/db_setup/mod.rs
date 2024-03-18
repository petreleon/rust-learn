// src/config/db_setup/mod.rs

pub mod updates;

use diesel::{dsl::max, prelude::*};
use diesel::pg::PgConnection;
use crate::db::schema::db_version_control::dsl::*;
use self::updates::{apply_update_v1};

pub fn version_updater(conn: &mut PgConnection) -> QueryResult<()> {

    let current_version_result = db_version_control
        .select(max(version))
        .first::<Option<i32>>(conn);

    let current_version = match current_version_result {
        Ok(Some(ver)) => ver,
        _ => 0,
    };

    let updates = vec![
        (1, apply_update_v1),
    ];

    for (target_version, update_function) in updates {
        if current_version < target_version {
            update_function(conn)?;
            diesel::update(db_version_control.filter(id.eq(1)))
                .set(version.eq(target_version))
                .execute(conn)?;
        }
    }

    Ok(())
}
