import { type OrganizationWorkspaceItem } from "./OrganizationWorkspaceItem";

export function organizationHasManagementScope(organization: OrganizationWorkspaceItem) {
  return (
    organization.effectivePermissions.some((permission) =>
      ["ASSIGN_ROLES_TO_ORG_USERS", "INVITE_USER_TO_ORGANIZATION", "MANAGE_ORG_MEMBERS"].includes(permission),
    ) ||
    organization.capabilities.some(
      (capability) =>
        capability.enabled &&
        (capability.key === "teacher_applications" || capability.key === "settings"),
    )
  );
}
