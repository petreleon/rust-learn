// src/config/db_setup/mod.rs

pub mod updates;

use diesel::{dsl::max, prelude::*};
use diesel::pg::PgConnection;
use crate::db::schema::db_version_control::dsl::*;
use self::updates::{apply_update_v1, apply_update_v2};

// A small alias for update functions stored in the update list.
// Each update receives a mutable Postgres connection and returns
// Diesel's `QueryResult<()>` (alias for Result<_, diesel::result::Error>).
type UpdateFn = fn(&mut PgConnection) -> QueryResult<()>;

/// Returns the list of available updates as pairs of (target_version, function).
///
/// Important: the list should be ordered by ascending version so updates are
/// applied incrementally. If you add more updates, keep them in ascending order.
fn updates() -> Vec<(i32, UpdateFn)> {
    vec![
        (1, apply_update_v1),
        (2, apply_update_v2),
    ]
}

/// Ensure the database has the latest version applied.
///
/// Behavior:
/// - Reads the current version from `db_version_control` (uses 0 when none).
/// - Applies each update whose target version is greater than the current version.
/// - At the end, sets the stored version to the maximum available update version
///   (no change if there are no updates or max <= current).
pub fn version_updater(conn: &mut PgConnection) -> QueryResult<()> {
    // Query the current version row. If it's missing or null, treat as 0.
    let current_version_opt: Option<i32> = db_version_control
        .select(max(version))
        .first(conn)?;

    let current_version = current_version_opt.unwrap_or(0);

    let updates = updates();

    if updates.is_empty() {
        // Nothing to do.
        return Ok(());
    }

    // Apply any updates that are newer than the current version.
    for (target_version, update_fn) in &updates {
        if current_version < *target_version {
            // Each update can return a Diesel error which we propagate up.
            update_fn(conn)?;
        }
    }

    // Determine the highest available version from the list.
    let max_version = updates.iter().map(|(v, _)| *v).max().unwrap_or(current_version);

    // Only write back if we advanced (or if the available max is greater).
    if max_version > current_version {
        diesel::update(db_version_control.filter(id.eq(1)))
            .set(version.eq(max_version))
            .execute(conn)?;
    }

    Ok(())
}
