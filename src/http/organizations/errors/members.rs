use crate::application::organizations::assign_organization_member_role::OrganizationMemberRoleAssignmentError;
use crate::application::organizations::invite_organization_member::OrganizationMemberInviteError;
use crate::application::organizations::list_organization_member_audit::OrganizationMemberAuditError;
use crate::application::organizations::list_organization_members::OrganizationMemberListError;
use crate::application::organizations::remove_organization_member::OrganizationMemberRemovalError;
use crate::http::errors::ApiError;

pub(in crate::http::organizations) fn organization_member_list_error(
    organization_id: i32,
    error: OrganizationMemberListError,
) -> ApiError {
    match error {
        OrganizationMemberListError::PermissionDenied => {
            super::permission_denied("User does not have permission to view organization members")
        }
        OrganizationMemberListError::NotFound => super::organization_not_found(),
        OrganizationMemberListError::Connection(error) => super::db_connection_failed(
            "organization_members_connection_failed",
            format!("organization_id={organization_id}"),
            error,
        ),
        OrganizationMemberListError::Database(error) => super::logged_internal(
            "organization_members_fetch_failed",
            &format!("organization_id={organization_id}"),
            "Failed to fetch organization members",
            error,
        ),
    }
}

pub(in crate::http::organizations) fn organization_member_audit_error(
    organization_id: i32,
    error: OrganizationMemberAuditError,
) -> ApiError {
    match error {
        OrganizationMemberAuditError::PermissionDenied(_) => super::permission_denied(
            "User does not have permission to view organization member audit",
        ),
        OrganizationMemberAuditError::Connection(error) => super::db_connection_failed(
            "member_audit_connection_failed",
            format!("organization_id={organization_id}"),
            error,
        ),
        OrganizationMemberAuditError::Database(error) => super::logged_internal(
            "member_audit_fetch_failed",
            &format!("organization_id={organization_id}"),
            "Failed to load audit events",
            error,
        ),
    }
}

pub(in crate::http::organizations) fn organization_member_removal_error(
    organization_id: i32,
    target_user_id: i32,
    error: OrganizationMemberRemovalError,
) -> ApiError {
    match error {
        OrganizationMemberRemovalError::PermissionDenied => {
            super::permission_denied("User does not have permission to remove organization members")
        }
        OrganizationMemberRemovalError::NotFound => {
            super::user_not_found("User not found in organization")
        }
        OrganizationMemberRemovalError::Connection(error) => super::db_connection_failed(
            "organization_member_remove_connection_failed",
            format!("organization_id={organization_id} target_user_id={target_user_id}"),
            error,
        ),
        OrganizationMemberRemovalError::Database(error) => super::logged_internal(
            "organization_member_remove_failed",
            &format!("organization_id={organization_id} target_user_id={target_user_id}"),
            "Failed to remove member",
            error,
        ),
    }
}

pub(in crate::http::organizations) fn organization_member_invite_error(
    organization_id: i32,
    email: &str,
    error: OrganizationMemberInviteError,
) -> ApiError {
    match error {
        OrganizationMemberInviteError::PermissionDenied => super::permission_denied(
            "User does not have the required permission within the organization",
        ),
        OrganizationMemberInviteError::UserNotFound => {
            super::user_not_found("User not found by email")
        }
        OrganizationMemberInviteError::HierarchyDenied => {
            super::hierarchy_denied("Hierarchy check failed")
        }
        OrganizationMemberInviteError::RoleOrUserNotFound => super::logged_internal(
            "org_member_add_role_lookup_failed",
            &format!("org_id={organization_id} email={email}"),
            "Failed to add member",
            "role or user not found".to_string(),
        ),
        OrganizationMemberInviteError::Connection(error) => super::db_connection_failed(
            "org_member_add_connection_failed",
            format!("org_id={organization_id} email={email}"),
            error,
        ),
        OrganizationMemberInviteError::Database(error) => super::logged_internal(
            "org_member_add_failed",
            &format!("org_id={organization_id} email={email}"),
            "Failed to add member",
            error,
        ),
    }
}

pub(in crate::http::organizations) fn organization_member_role_assignment_error(
    error: OrganizationMemberRoleAssignmentError,
    organization_id: i32,
    actor_user_id: i32,
    target_user_id: i32,
    role_name: &str,
) -> ApiError {
    let context = format!(
        "organization_id={organization_id} requester_user_id={actor_user_id} target_user_id={target_user_id} role={role_name}"
    );
    match error {
        OrganizationMemberRoleAssignmentError::PermissionDenied => super::permission_denied(
            "User does not have the required permission within the organization",
        ),
        OrganizationMemberRoleAssignmentError::HierarchyViolation => super::hierarchy_denied(
            "Hierarchy check failed: Cannot assign role higher than or equal to your own, or modify user with higher/equal rank.",
        ),
        OrganizationMemberRoleAssignmentError::NotFound => super::invalid_input("Role or User not found"),
        OrganizationMemberRoleAssignmentError::Connection(error) => super::db_connection_failed(
            "organization_role_assign_connection_failed",
            context,
            error,
        ),
        OrganizationMemberRoleAssignmentError::Database(error) => super::logged_internal(
            "organization_role_assign_failed",
            &context,
            "Failed to assign role",
            error,
        ),
    }
}
