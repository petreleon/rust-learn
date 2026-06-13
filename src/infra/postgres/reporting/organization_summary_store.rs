use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::reporting::organization_summary::store::OrganizationSummaryStore;
use crate::application::reporting::organization_summary::{
    OrganizationSummaryError, OrganizationSummaryOutput,
};
use crate::db::schema::{
    courses_organizations, organizations, user_role_course, user_role_organization, wallets,
};

pub struct PostgresOrganizationSummaryStore<'a> {
    conn: &'a mut AsyncPgConnection,
}

impl<'a> PostgresOrganizationSummaryStore<'a> {
    pub fn new(conn: &'a mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl OrganizationSummaryStore for PostgresOrganizationSummaryStore<'_> {
    fn load_organization_summary(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<OrganizationSummaryOutput, OrganizationSummaryError>> {
        async move {
            let organization_name = organizations::table
                .find(organization_id)
                .select(organizations::name)
                .first::<String>(self.conn)
                .await
                .map_err(map_diesel_error)?;
            let course_ids = courses_organizations::table
                .filter(courses_organizations::organization_id.eq(organization_id))
                .select(courses_organizations::course_id)
                .load::<i32>(self.conn)
                .await
                .map_err(map_diesel_error)?;
            let member_ids = user_role_organization::table
                .filter(user_role_organization::organization_id.eq(organization_id))
                .select(user_role_organization::user_id)
                .distinct()
                .load::<Option<i32>>(self.conn)
                .await
                .map_err(map_diesel_error)?;
            let wallet_count = wallets::table
                .filter(wallets::organization_id.eq(organization_id))
                .count()
                .get_result(self.conn)
                .await
                .map_err(map_diesel_error)?;
            let course_role_assignment_count =
                course_role_assignment_count(self.conn, &course_ids).await?;

            Ok(OrganizationSummaryOutput {
                organization_id,
                organization_name,
                course_count: course_ids.len() as i64,
                member_count: member_ids.into_iter().flatten().count() as i64,
                wallet_count,
                course_role_assignment_count,
            })
        }
        .boxed()
    }
}

async fn course_role_assignment_count(
    conn: &mut AsyncPgConnection,
    course_ids: &[i32],
) -> Result<i64, OrganizationSummaryError> {
    if course_ids.is_empty() {
        return Ok(0);
    }
    user_role_course::table
        .filter(user_role_course::course_id.eq_any(course_ids))
        .count()
        .get_result(conn)
        .await
        .map_err(map_diesel_error)
}

fn map_diesel_error(error: diesel::result::Error) -> OrganizationSummaryError {
    match error {
        diesel::result::Error::NotFound => OrganizationSummaryError::NotFound,
        other => OrganizationSummaryError::Database(other.to_string()),
    }
}
