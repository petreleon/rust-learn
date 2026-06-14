use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::reporting::organization_summary::store::OrganizationSummaryStore;
use crate::application::reporting::organization_summary::{
    organization_summary_from_facts, OrganizationSummaryError, OrganizationSummaryFacts,
    OrganizationSummaryOutput,
};
use crate::infra::postgres::reporting::organization_summary_course_roles::course_role_assignment_count;
use crate::infra::postgres::reporting::organization_summary_courses::organization_course_ids;
use crate::infra::postgres::reporting::organization_summary_members::organization_member_user_ids;
use crate::infra::postgres::reporting::organization_summary_organizations::organization_name;
use crate::infra::postgres::reporting::organization_summary_wallets::organization_wallet_count;

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
            let organization_name = organization_name(self.conn, organization_id).await?;
            let course_ids = organization_course_ids(self.conn, organization_id).await?;
            let member_ids = organization_member_user_ids(self.conn, organization_id).await?;
            let wallet_count = organization_wallet_count(self.conn, organization_id).await?;
            let course_role_assignment_count =
                course_role_assignment_count(self.conn, &course_ids).await?;

            Ok(organization_summary_from_facts(OrganizationSummaryFacts {
                organization_id,
                organization_name,
                course_ids,
                member_user_ids: member_ids,
                wallet_count,
                course_role_assignment_count,
            }))
        }
        .boxed()
    }
}
