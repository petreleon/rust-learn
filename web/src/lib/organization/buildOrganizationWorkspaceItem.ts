import { type OrganizationSessionScope, type SessionCapability } from "@/lib/session";
import { type OrganizationCapabilityKey } from "./OrganizationCapabilityKey";
import { type OrganizationWorkspaceItem } from "./OrganizationWorkspaceItem";

const organizationCapabilityKeys = new Set<string>([
  "courses",
  "members",
  "reports",
  "member_management",
  "wallet",
  "teacher_applications",
  "course_rewards",
  "settings",
]);

function hasOrganizationCapabilityKey(
  capability: SessionCapability,
): capability is SessionCapability & { key: OrganizationCapabilityKey } {
  return organizationCapabilityKeys.has(capability.key);
}

export function buildOrganizationWorkspaceItem(organization: OrganizationSessionScope): OrganizationWorkspaceItem {
  return {
    capabilities: organization.capabilities
      .filter(hasOrganizationCapabilityKey)
      .map((capability) => ({ ...capability, key: capability.key })),
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
