import { type OrganizationWorkspaceItem } from "./OrganizationWorkspaceItem";

export function organizationPermissionEnabled(
  organization: OrganizationWorkspaceItem | null | undefined,
  permission: string,
) {
  if (!organization) {
    return false;
  }

  return (
    organization.effectivePermissions.includes(permission) &&
    organization.capabilities.some(
      (capability) => capability.enabled && capability.permissions.includes(permission),
    )
  );
}
