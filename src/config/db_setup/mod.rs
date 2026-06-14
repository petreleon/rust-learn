// src/config/db_setup/mod.rs

pub mod updates;

use self::updates::{apply_update_v1, apply_update_v2};
use crate::infra::postgres::operations::db_version_control;
use anyhow::Result;
use diesel_async::AsyncPgConnection;
use futures::future::BoxFuture;

// A small alias for update functions stored in the update list.
// Each update receives a mutable Postgres connection and returns
// an error with enough context for clean startup logs.
type UpdateFn = fn(&mut AsyncPgConnection) -> BoxFuture<'_, Result<()>>;

/// Returns the list of available updates as pairs of (target_version, function).
///
/// Important: the list should be ordered by ascending version so updates are
/// applied incrementally. If you add more updates, keep them in ascending order.
fn updates() -> Vec<(i32, UpdateFn)> {
    vec![(1, apply_update_v1), (2, apply_update_v2)]
}

/// Ensure the database has the latest version applied.
///
/// Behavior:
/// - Reads the current version from `db_version_control` (uses 0 when none).
/// - Applies each update whose target version is greater than the current version.
/// - At the end, sets the stored version to the maximum available update version
///   (no change if there are no updates or max <= current).
pub async fn version_updater(conn: &mut AsyncPgConnection) -> Result<()> {
    // Query the current version row. If it's missing or null, treat as 0.
    let current_version = db_version_control::get_current_version(conn).await?;

    let updates = updates();

    if updates.is_empty() {
        // Nothing to do.
        return Ok(());
    }

    // Apply any updates that are newer than the current version.
    for (target_version, update_fn) in &updates {
        if current_version < *target_version {
            // Each update can return a Diesel error which we propagate up.
            update_fn(conn).await?;
        }
    }

    // Determine the highest available version from the list.
    let max_version = updates
        .iter()
        .map(|(v, _)| *v)
        .max()
        .unwrap_or(current_version);

    // Only write back if we advanced (or if the available max is greater).
    if max_version > current_version {
        db_version_control::update_version(conn, max_version).await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn updates_list_is_ordered_and_not_empty() {
        let updates_list = updates();
        assert!(!updates_list.is_empty());

        let mut prev = 0;
        for (version, _) in &updates_list {
            assert!(*version > prev, "updates must be in ascending order");
            prev = *version;
        }
    }

    #[test]
    fn updates_list_contains_expected_versions() {
        let versions: Vec<i32> = updates().into_iter().map(|(v, _)| v).collect();
        assert!(versions.contains(&1));
        assert!(versions.contains(&2));
    }
}
