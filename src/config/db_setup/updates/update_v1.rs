// src/config/db_setup/updates/update_v1.rs

use anyhow::Result;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

pub fn apply_update_v1(_conn: &mut AsyncPgConnection) -> BoxFuture<'_, Result<()>> {
    async move {
        println!("Applying update v1...");
        Ok(())
    }
    .boxed()
}
