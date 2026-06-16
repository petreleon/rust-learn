import { type OrganizationSessionScope, type SessionCapability } from "@/lib/session";
import { type OrganizationCapability } from "./OrganizationCapability";
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
  const capabilities = organization.capabilities?.length
    ? organization.capabilities
    : legacyOrganizationCapabilities(organization.effective_permissions);

  return {
    capabilities: capabilities
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

function legacyOrganizationCapabilities(permissions: string[]): OrganizationCapability[] {
  const enabled = new Set(permissions);
  return [
    capability("courses", "Courses", ["VIEW_ORGANIZATION"], enabled),
    capability("members", "Members", ["INVITE_USER_TO_ORGANIZATION"], enabled),
    capability("reports", "Reports", ["VIEW_ORG_REWARD_REPORTS"], enabled),
    capability("member_management", "Member management", ["INVITE_USER_TO_ORGANIZATION"], enabled),
    capability("wallet", "Wallet", ["MANAGE_ORG_WALLETS"], enabled),
    capability("teacher_applications", "Teacher applications", ["NOMINATE_TEACHER_FOR_PLATFORM_REVIEW"], enabled),
    capability("course_rewards", "Course rewards", ["SUBMIT_ORG_COURSE_REWARD_EVENT"], enabled),
    capability("settings", "Settings", ["INVITE_USER_TO_ORGANIZATION"], enabled),
  ];
}

function capability(
  key: OrganizationCapabilityKey,
  label: string,
  permissions: string[],
  enabled: Set<string>,
): OrganizationCapability {
  return {
    enabled: permissions.some((permission) => enabled.has(permission)),
    key,
    label,
    permissions,
  };
}
