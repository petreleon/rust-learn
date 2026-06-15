use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::reporting::platform_summary::store::PlatformSummaryStore;
use crate::application::reporting::platform_summary::{
    platform_summary_output, PlatformSummaryError, PlatformSummaryFact, PlatformSummaryOutput,
};
use crate::infra::postgres::schema::{courses, notifications, organizations, users, wallets};

pub struct PostgresPlatformSummaryStore<'a> {
    conn: &'a mut AsyncPgConnection,
}

impl<'a> PostgresPlatformSummaryStore<'a> {
    pub fn new(conn: &'a mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl PlatformSummaryStore for PostgresPlatformSummaryStore<'_> {
    fn load_platform_summary(
        &mut self,
    ) -> BoxFuture<'_, Result<PlatformSummaryOutput, PlatformSummaryError>> {
        async move {
            let total_users = users::table
                .count()
                .get_result(self.conn)
                .await
                .map_err(map_diesel_error)?;
            let total_organizations = organizations::table
                .count()
                .get_result(self.conn)
                .await
                .map_err(map_diesel_error)?;
            let total_courses = courses::table
                .count()
                .get_result(self.conn)
                .await
                .map_err(map_diesel_error)?;
            let total_wallets = wallets::table
                .count()
                .get_result(self.conn)
                .await
                .map_err(map_diesel_error)?;
            let total_notifications = notifications::table
                .count()
                .get_result(self.conn)
                .await
                .map_err(map_diesel_error)?;

            Ok(platform_summary_output(PlatformSummaryFact {
                total_users,
                total_organizations,
                total_courses,
                total_wallets,
                total_notifications,
            }))
        }
        .boxed()
    }
}

fn map_diesel_error(error: diesel::result::Error) -> PlatformSummaryError {
    PlatformSummaryError::Database(error.to_string())
}
