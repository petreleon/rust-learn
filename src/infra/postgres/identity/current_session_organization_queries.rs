use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::db::schema::{
    organization_roles, organizations, role_permission_organization, user_role_organization,
};

#[derive(Debug)]
pub(super) struct OrganizationRoleRow {
    pub(super) organization_id: i32,
    pub(super) organization_name: String,
    pub(super) role_name: String,
}

#[derive(Debug)]
pub(super) struct OrganizationPermissionRow {
    pub(super) organization_id: i32,
    pub(super) organization_name: String,
    pub(super) permission: String,
}

pub(super) async fn list_organization_roles(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> QueryResult<Vec<OrganizationRoleRow>> {
    let rows =
        user_role_organization::table
            .inner_join(
                organizations::table
                    .on(user_role_organization::organization_id.eq(organizations::id.nullable())),
            )
            .inner_join(organization_roles::table.on(
                user_role_organization::organization_role_id.eq(organization_roles::id.nullable()),
            ))
            .filter(user_role_organization::user_id.eq(user_id))
            .select((
                organizations::id,
                organizations::name,
                organization_roles::name,
            ))
            .load::<(i32, String, String)>(conn)
            .await?;

    Ok(rows
        .into_iter()
        .map(
            |(organization_id, organization_name, role_name)| OrganizationRoleRow {
                organization_id,
                organization_name,
                role_name,
            },
        )
        .collect())
}

pub(super) async fn list_organization_permissions(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> QueryResult<Vec<OrganizationPermissionRow>> {
    let rows = user_role_organization::table
        .inner_join(
            organizations::table
                .on(user_role_organization::organization_id.eq(organizations::id.nullable())),
        )
        .inner_join(
            role_permission_organization::table.on(user_role_organization::organization_role_id
                .eq(role_permission_organization::organization_role_id)),
        )
        .filter(user_role_organization::user_id.eq(user_id))
        .select((
            organizations::id,
            organizations::name,
            role_permission_organization::permission,
        ))
        .load::<(i32, String, String)>(conn)
        .await?;

    Ok(rows
        .into_iter()
        .map(
            |(organization_id, organization_name, permission)| OrganizationPermissionRow {
                organization_id,
                organization_name,
                permission,
            },
        )
        .collect())
}
