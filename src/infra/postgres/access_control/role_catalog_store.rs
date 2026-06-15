use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::access_control::ports::RoleCatalogStore;
use crate::application::access_control::role_catalog::{
    RoleCatalogEntry, RoleCatalogError, RoleCatalogScope,
};
use crate::infra::postgres::models::role::{CourseRole, OrganizationRole, PlatformRole};
use crate::infra::postgres::schema::{course_roles, organization_roles, platform_roles};

pub struct PostgresRoleCatalogStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresRoleCatalogStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

pub async fn platform_role_id_by_name(
    conn: &mut AsyncPgConnection,
    role_name: &str,
) -> QueryResult<i32> {
    platform_roles::table
        .filter(platform_roles::name.eq(role_name))
        .select(platform_roles::id)
        .first::<i32>(conn)
        .await
}

pub async fn organization_role_id_by_name(
    conn: &mut AsyncPgConnection,
    role_name: &str,
) -> QueryResult<i32> {
    organization_roles::table
        .filter(organization_roles::name.eq(role_name))
        .select(organization_roles::id)
        .first::<i32>(conn)
        .await
}

pub async fn course_role_id_by_name(
    conn: &mut AsyncPgConnection,
    role_name: &str,
) -> QueryResult<i32> {
    course_roles::table
        .filter(course_roles::name.eq(role_name))
        .select(course_roles::id)
        .first::<i32>(conn)
        .await
}

impl RoleCatalogStore for PostgresRoleCatalogStore<'_> {
    fn list_roles(
        &mut self,
        scope: RoleCatalogScope,
    ) -> BoxFuture<'_, Result<Vec<RoleCatalogEntry>, RoleCatalogError>> {
        async move {
            match scope {
                RoleCatalogScope::Platform => platform_roles::table
                    .load::<PlatformRole>(self.conn)
                    .await
                    .map(|roles| roles.into_iter().map(RoleCatalogEntry::from).collect()),
                RoleCatalogScope::Organization => organization_roles::table
                    .load::<OrganizationRole>(self.conn)
                    .await
                    .map(|roles| roles.into_iter().map(RoleCatalogEntry::from).collect()),
                RoleCatalogScope::Course => course_roles::table
                    .load::<CourseRole>(self.conn)
                    .await
                    .map(|roles| roles.into_iter().map(RoleCatalogEntry::from).collect()),
            }
            .map_err(map_role_catalog_error)
        }
        .boxed()
    }
}

impl From<PlatformRole> for RoleCatalogEntry {
    fn from(role: PlatformRole) -> Self {
        Self {
            id: role.id,
            name: role.name,
            description: role.description,
        }
    }
}

impl From<OrganizationRole> for RoleCatalogEntry {
    fn from(role: OrganizationRole) -> Self {
        Self {
            id: role.id,
            name: role.name,
            description: role.description,
        }
    }
}

impl From<CourseRole> for RoleCatalogEntry {
    fn from(role: CourseRole) -> Self {
        Self {
            id: role.id,
            name: role.name,
            description: role.description,
        }
    }
}

fn map_role_catalog_error(error: diesel::result::Error) -> RoleCatalogError {
    RoleCatalogError::Database(error.to_string())
}
