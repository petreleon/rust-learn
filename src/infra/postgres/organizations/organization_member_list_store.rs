use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use futures::future::{BoxFuture, FutureExt};

use crate::application::organizations::list_organization_members::{
    OrganizationMemberListError, OrganizationMemberListItemOutput, OrganizationMemberListOutput,
    OrganizationMemberListQuery, OrganizationMemberListStore, OrganizationMemberOrganizationOutput,
};
use crate::db::schema::organizations;
use crate::infra::postgres::models::organization::Organization;
use crate::infra::postgres::organizations::{
    organization_member_builders, organization_member_permission_queries,
};

pub struct PostgresOrganizationMemberListStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresOrganizationMemberListStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl OrganizationMemberListStore for PostgresOrganizationMemberListStore<'_> {
    fn list_organization_members(
        &mut self,
        query: OrganizationMemberListQuery,
    ) -> BoxFuture<'_, Result<OrganizationMemberListOutput, OrganizationMemberListError>> {
        async move {
            let organization = load_organization(self.conn, query.organization_id).await?;
            if !organization_member_permission_queries::can_view_organization_members(
                self.conn,
                query.actor_user_id,
                organization.id,
            )
            .await?
            {
                return Err(OrganizationMemberListError::PermissionDenied);
            }

            let operator_permissions =
                organization_member_permission_queries::build_member_operator_permissions(
                    self.conn,
                    query.actor_user_id,
                    organization.id,
                )
                .await?;
            let mut members = organization_member_builders::build_organization_member_builders(
                self.conn,
                organization.id,
            )
            .await?;
            organization_member_permission_queries::attach_member_permissions(
                self.conn,
                organization.id,
                &mut members,
            )
            .await?;
            organization_member_permission_queries::attach_member_delegations(
                self.conn,
                organization.id,
                &mut members,
            )
            .await?;

            let mut items = members
                .into_values()
                .map(organization_member_builders::member_output_from_builder)
                .filter(|member| member_matches_query(member, &query))
                .collect::<Vec<_>>();
            sort_members(&mut items);

            let total = items.len() as i64;
            let members = items
                .into_iter()
                .skip(query.offset as usize)
                .take(query.limit as usize)
                .collect();

            Ok(OrganizationMemberListOutput {
                organization,
                members,
                total,
                limit: query.limit,
                offset: query.offset,
                search: query.search,
                role: query.role,
                permission: query.permission,
                operator_permissions,
            })
        }
        .boxed()
    }
}

fn member_matches_query(
    member: &OrganizationMemberListItemOutput,
    query: &OrganizationMemberListQuery,
) -> bool {
    matches_search(member, query.search.as_deref())
        && matches_role(member, query.role.as_deref())
        && matches_permission(member, query.permission.as_deref())
}

fn matches_search(member: &OrganizationMemberListItemOutput, search: Option<&str>) -> bool {
    search
        .map(|search| {
            let normalized = search.to_lowercase();
            member.name.to_lowercase().contains(&normalized)
                || member.email.to_lowercase().contains(&normalized)
                || member
                    .roles
                    .iter()
                    .any(|role| role.to_lowercase().contains(&normalized))
                || member
                    .effective_permissions
                    .iter()
                    .any(|permission| permission.to_lowercase().contains(&normalized))
        })
        .unwrap_or(true)
}

fn matches_role(member: &OrganizationMemberListItemOutput, role: Option<&str>) -> bool {
    role.map(|role| {
        member
            .roles
            .iter()
            .any(|member_role| member_role.eq_ignore_ascii_case(role))
    })
    .unwrap_or(true)
}

fn matches_permission(member: &OrganizationMemberListItemOutput, permission: Option<&str>) -> bool {
    permission
        .map(|permission| {
            member
                .effective_permissions
                .iter()
                .any(|member_permission| member_permission.eq_ignore_ascii_case(permission))
        })
        .unwrap_or(true)
}

fn sort_members(items: &mut [OrganizationMemberListItemOutput]) {
    items.sort_by(|left, right| {
        left.name
            .to_lowercase()
            .cmp(&right.name.to_lowercase())
            .then_with(|| left.email.to_lowercase().cmp(&right.email.to_lowercase()))
            .then_with(|| left.id.cmp(&right.id))
    });
}

async fn load_organization(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<OrganizationMemberOrganizationOutput, OrganizationMemberListError> {
    organizations::table
        .find(organization_id)
        .first::<Organization>(conn)
        .await
        .map(|organization| OrganizationMemberOrganizationOutput {
            id: organization.id,
            name: organization.name,
        })
        .map_err(map_member_error)
}

fn map_member_error(error: diesel::result::Error) -> OrganizationMemberListError {
    match error {
        diesel::result::Error::NotFound => OrganizationMemberListError::NotFound,
        other => OrganizationMemberListError::Database(other.to_string()),
    }
}
