use anyhow::Result;
use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

pub fn apply_update_v1(_conn: &mut AsyncPgConnection) -> BoxFuture<'_, Result<()>> {
    async move {
        log::info!("event=db_version_update_apply_started version=1");
        Ok(())
    }
    .boxed()
}
