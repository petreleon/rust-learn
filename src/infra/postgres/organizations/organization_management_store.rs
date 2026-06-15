use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::organizations::manage_organizations::{
    OrganizationCreateCommand, OrganizationManagementError, OrganizationManagementStore,
    OrganizationOutput, OrganizationUpdateCommand,
};
use crate::infra::postgres::models::courses_organizations::NewCourseOrganization;
use crate::infra::postgres::models::organization::{
    NewOrganization, Organization, UpdateOrganization,
};
use crate::infra::postgres::schema::{courses_organizations, organizations};

pub struct PostgresOrganizationManagementStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresOrganizationManagementStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl OrganizationManagementStore for PostgresOrganizationManagementStore<'_> {
    fn list_organizations(
        &mut self,
    ) -> BoxFuture<'_, Result<Vec<OrganizationOutput>, OrganizationManagementError>> {
        async move {
            organizations::table
                .load::<Organization>(self.conn)
                .await
                .map(|items| items.into_iter().map(Into::into).collect())
                .map_err(map_error)
        }
        .boxed()
    }

    fn get_organization(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<OrganizationOutput, OrganizationManagementError>> {
        async move {
            organizations::table
                .find(organization_id)
                .first::<Organization>(self.conn)
                .await
                .map(Into::into)
                .map_err(map_error)
        }
        .boxed()
    }

    fn create_organization(
        &mut self,
        command: OrganizationCreateCommand,
    ) -> BoxFuture<'_, Result<OrganizationOutput, OrganizationManagementError>> {
        async move { create_organization(self.conn, command).await }.boxed()
    }

    fn update_organization(
        &mut self,
        command: OrganizationUpdateCommand,
    ) -> BoxFuture<'_, Result<OrganizationOutput, OrganizationManagementError>> {
        async move {
            let update = UpdateOrganization {
                name: command.name,
                website_link: command.website_link,
                profile_url: command.profile_url,
            };
            diesel::update(organizations::table.find(command.organization_id))
                .set(&update)
                .get_result::<Organization>(self.conn)
                .await
                .map(Into::into)
                .map_err(map_error)
        }
        .boxed()
    }

    fn delete_organization(
        &mut self,
        organization_id: i32,
    ) -> BoxFuture<'_, Result<bool, OrganizationManagementError>> {
        async move {
            diesel::delete(organizations::table.find(organization_id))
                .execute(self.conn)
                .await
                .map(|count| count > 0)
                .map_err(map_error)
        }
        .boxed()
    }
}

async fn create_organization(
    conn: &mut AsyncPgConnection,
    command: OrganizationCreateCommand,
) -> Result<OrganizationOutput, OrganizationManagementError> {
    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        Box::pin(async move {
            let organization = diesel::insert_into(organizations::table)
                .values(NewOrganization {
                    name: command.name,
                    website_link: command.website_link,
                    profile_url: command.profile_url,
                })
                .get_result::<Organization>(conn)
                .await?;

            if let Some(course_ids) = command.course_ids {
                for (index, course_id) in course_ids.iter().enumerate() {
                    diesel::insert_into(courses_organizations::table)
                        .values(NewCourseOrganization {
                            course_id: *course_id,
                            organization_id: organization.id,
                            order: index as i32,
                        })
                        .execute(conn)
                        .await?;
                }
            }

            Ok(organization)
        })
    })
    .await
    .map(Into::into)
    .map_err(map_error)
}

impl From<Organization> for OrganizationOutput {
    fn from(organization: Organization) -> Self {
        Self {
            id: organization.id,
            name: organization.name,
            website_link: organization.website_link,
            profile_url: organization.profile_url,
        }
    }
}

fn map_error(error: diesel::result::Error) -> OrganizationManagementError {
    match error {
        diesel::result::Error::NotFound => OrganizationManagementError::NotFound,
        other => OrganizationManagementError::Database(other.to_string()),
    }
}
