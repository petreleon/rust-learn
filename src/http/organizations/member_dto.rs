use serde::Serialize;

use crate::application::organizations::list_organization_members::{
    OrganizationMemberListItemOutput, OrganizationMemberListOutput,
    OrganizationMemberOperatorPermissionsOutput, OrganizationMemberOrganizationOutput,
};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationMemberListResponse {
    pub organization: OrganizationMemberOrganizationResponse,
    pub members: Vec<OrganizationMemberListItemResponse>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub search: Option<String>,
    pub role: Option<String>,
    pub permission: Option<String>,
    pub operator_permissions: OrganizationMemberOperatorPermissionsResponse,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationMemberOrganizationResponse {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationMemberListItemResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub kyc_verified: bool,
    pub joined_at: String,
    pub roles: Vec<String>,
    pub direct_permissions: Vec<String>,
    pub delegated_permissions: Vec<String>,
    pub effective_permissions: Vec<String>,
    pub direct_permission_count: usize,
    pub delegated_permission_count: usize,
    pub effective_permission_count: usize,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationMemberOperatorPermissionsResponse {
    pub can_view_members: bool,
    pub can_invite_members: bool,
    pub can_manage_members: bool,
    pub can_assign_roles: bool,
    pub can_manage_settings: bool,
}

impl From<OrganizationMemberListOutput> for OrganizationMemberListResponse {
    fn from(output: OrganizationMemberListOutput) -> Self {
        Self {
            organization: output.organization.into(),
            members: output.members.into_iter().map(Into::into).collect(),
            total: output.total,
            limit: output.limit,
            offset: output.offset,
            search: output.search,
            role: output.role,
            permission: output.permission,
            operator_permissions: output.operator_permissions.into(),
        }
    }
}

impl From<OrganizationMemberOrganizationOutput> for OrganizationMemberOrganizationResponse {
    fn from(organization: OrganizationMemberOrganizationOutput) -> Self {
        Self {
            id: organization.id,
            name: organization.name,
        }
    }
}

impl From<OrganizationMemberListItemOutput> for OrganizationMemberListItemResponse {
    fn from(member: OrganizationMemberListItemOutput) -> Self {
        Self {
            id: member.id,
            name: member.name,
            email: member.email,
            email_verified: member.email_verified,
            kyc_verified: member.kyc_verified,
            joined_at: member.joined_at,
            roles: member.roles,
            direct_permissions: member.direct_permissions,
            delegated_permissions: member.delegated_permissions,
            effective_permissions: member.effective_permissions,
            direct_permission_count: member.direct_permission_count,
            delegated_permission_count: member.delegated_permission_count,
            effective_permission_count: member.effective_permission_count,
        }
    }
}

impl From<OrganizationMemberOperatorPermissionsOutput>
    for OrganizationMemberOperatorPermissionsResponse
{
    fn from(permissions: OrganizationMemberOperatorPermissionsOutput) -> Self {
        Self {
            can_view_members: permissions.can_view_members,
            can_invite_members: permissions.can_invite_members,
            can_manage_members: permissions.can_manage_members,
            can_assign_roles: permissions.can_assign_roles,
            can_manage_settings: permissions.can_manage_settings,
        }
    }
}
