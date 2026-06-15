use std::collections::{BTreeMap, BTreeSet};

use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::organizations::list_organization_members::{
    OrganizationMemberListError, OrganizationMemberListItemOutput,
};
use crate::infra::postgres::schema::{organization_roles, user_role_organization, users};

#[derive(Debug)]
pub(super) struct OrganizationMemberBuilder {
    pub(super) id: i32,
    pub(super) name: String,
    pub(super) email: String,
    pub(super) email_verified: bool,
    pub(super) kyc_verified: bool,
    pub(super) joined_at: NaiveDateTime,
    pub(super) roles: BTreeSet<String>,
    pub(super) direct_permissions: BTreeSet<String>,
    pub(super) delegated_permissions: BTreeSet<String>,
}

pub(super) async fn build_organization_member_builders(
    conn: &mut AsyncPgConnection,
    organization_id: i32,
) -> Result<BTreeMap<i32, OrganizationMemberBuilder>, OrganizationMemberListError> {
    let rows =
        user_role_organization::table
            .inner_join(users::table.on(user_role_organization::user_id.eq(users::id.nullable())))
            .inner_join(organization_roles::table.on(
                user_role_organization::organization_role_id.eq(organization_roles::id.nullable()),
            ))
            .filter(user_role_organization::organization_id.eq(organization_id))
            .select((
                users::id,
                users::name,
                users::email,
                users::email_verified,
                users::kyc_verified,
                users::created_at,
                organization_roles::name,
            ))
            .load::<(i32, String, String, bool, bool, NaiveDateTime, String)>(conn)
            .await
            .map_err(map_member_error)?;

    let mut members = BTreeMap::<i32, OrganizationMemberBuilder>::new();
    for (user_id, name, email, email_verified, kyc_verified, created_at, role_name) in rows {
        members
            .entry(user_id)
            .or_insert_with(|| OrganizationMemberBuilder {
                id: user_id,
                name,
                email,
                email_verified,
                kyc_verified,
                joined_at: created_at,
                roles: BTreeSet::new(),
                direct_permissions: BTreeSet::new(),
                delegated_permissions: BTreeSet::new(),
            })
            .roles
            .insert(role_name);
    }

    Ok(members)
}

pub(super) fn member_output_from_builder(
    member: OrganizationMemberBuilder,
) -> OrganizationMemberListItemOutput {
    let direct_permissions = sorted_vec(member.direct_permissions);
    let delegated_permissions = sorted_vec(member.delegated_permissions);
    let effective_permissions = direct_permissions
        .iter()
        .chain(delegated_permissions.iter())
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let direct_permission_count = direct_permissions.len();
    let delegated_permission_count = delegated_permissions.len();
    let effective_permission_count = effective_permissions.len();

    OrganizationMemberListItemOutput {
        id: member.id,
        name: member.name,
        email: member.email,
        email_verified: member.email_verified,
        kyc_verified: member.kyc_verified,
        joined_at: member.joined_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
        roles: sorted_vec(member.roles),
        direct_permissions,
        delegated_permissions,
        effective_permissions,
        direct_permission_count,
        delegated_permission_count,
        effective_permission_count,
    }
}

fn sorted_vec(values: BTreeSet<String>) -> Vec<String> {
    values.into_iter().collect()
}

fn map_member_error(error: diesel::result::Error) -> OrganizationMemberListError {
    match error {
        diesel::result::Error::NotFound => OrganizationMemberListError::NotFound,
        other => OrganizationMemberListError::Database(other.to_string()),
    }
}
