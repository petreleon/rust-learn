import { type OrganizationSessionScope } from "@/lib/session";
import { capabilityPermissions } from "./capabilityPermissions";
import { type OrganizationWorkspaceItem } from "./OrganizationWorkspaceItem";

export function buildOrganizationWorkspaceItem(organization: OrganizationSessionScope): OrganizationWorkspaceItem {
  return {
    capabilities: capabilityPermissions.map((capability) => ({
      enabled: capability.permissions.some((permission) => organization.effective_permissions.includes(permission)),
      key: capability.key,
      label: capability.label,
      permissions: capability.permissions,
    })),
    delegatedPermissionCount: organization.delegated_permissions.length,
    directPermissionCount: organization.direct_permissions.length,
    effectivePermissions: [...organization.effective_permissions],
    effectivePermissionCount: organization.effective_permissions.length,
    id: organization.id,
    name: organization.name,
    permissionPreview: organization.effective_permissions.slice(0, 5),
    roles: organization.roles,
  };
}
