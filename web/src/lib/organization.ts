import type { CurrentSession, OrganizationSessionScope } from "@/lib/session";

export type OrganizationCapabilityKey =
  | "members"
  | "reports"
  | "wallet"
  | "teacher_applications"
  | "course_rewards"
  | "settings";

export type OrganizationCapability = {
  enabled: boolean;
  key: OrganizationCapabilityKey;
  label: string;
  permissions: string[];
};

export type OrganizationWorkspaceItem = {
  capabilities: OrganizationCapability[];
  delegatedPermissionCount: number;
  directPermissionCount: number;
  effectivePermissions: string[];
  effectivePermissionCount: number;
  id: number;
  name: string;
  permissionPreview: string[];
  roles: string[];
};

export type OrganizationWorkspaceSummary = {
  courseRewardScopeCount: number;
  delegatedOrganizationCount: number;
  managementScopeCount: number;
  organizations: OrganizationWorkspaceItem[];
  reportScopeCount: number;
  teacherNominationScopeCount: number;
  total: number;
  walletScopeCount: number;
};

export type OrganizationFilter = {
  capability: OrganizationCapabilityKey | "all" | "delegated";
  search: string;
};

const capabilityPermissions: Array<{
  key: OrganizationCapabilityKey;
  label: string;
  permissions: string[];
}> = [
  {
    key: "members",
    label: "Members",
    permissions: ["ASSIGN_ROLES_TO_ORG_USERS", "INVITE_USER_TO_ORGANIZATION", "MANAGE_ORG_MEMBERS"],
  },
  {
    key: "reports",
    label: "Reports",
    permissions: ["VIEW_ORG_REWARD_REPORTS"],
  },
  {
    key: "wallet",
    label: "Wallet",
    permissions: ["MANAGE_ORG_BILLING", "MANAGE_ORG_REWARD_BUDGET", "MANAGE_ORG_WALLETS"],
  },
  {
    key: "teacher_applications",
    label: "Teacher nominations",
    permissions: ["NOMINATE_TEACHER_FOR_PLATFORM_REVIEW", "VIEW_ORG_TEACHER_APPLICATIONS"],
  },
  {
    key: "course_rewards",
    label: "Course rewards",
    permissions: ["SUBMIT_ORG_COURSE_REWARD_EVENT"],
  },
  {
    key: "settings",
    label: "Settings",
    permissions: ["MANAGE_ORG_SETTINGS", "VIEW_ORGANIZATION"],
  },
];

export function buildOrganizationWorkspace(session: CurrentSession): OrganizationWorkspaceSummary {
  const organizations = session.organizations
    .filter(hasOrganizationScopeSignal)
    .map(buildOrganizationWorkspaceItem)
    .sort((left, right) => left.name.localeCompare(right.name));

  return {
    delegatedOrganizationCount: organizations.filter((organization) => organization.delegatedPermissionCount > 0).length,
    courseRewardScopeCount: organizations.filter((organization) =>
      organization.capabilities.some((capability) => capability.key === "course_rewards" && capability.enabled),
    ).length,
    managementScopeCount: organizations.filter((organization) =>
      organization.capabilities.some(
        (capability) =>
          capability.enabled &&
          (capability.key === "members" ||
            capability.key === "teacher_applications" ||
            capability.key === "settings"),
      ),
    ).length,
    organizations,
    reportScopeCount: organizations.filter((organization) =>
      organization.capabilities.some((capability) => capability.key === "reports" && capability.enabled),
    ).length,
    teacherNominationScopeCount: organizations.filter((organization) =>
      organization.capabilities.some((capability) => capability.key === "teacher_applications" && capability.enabled),
    ).length,
    total: organizations.length,
    walletScopeCount: organizations.filter((organization) =>
      organization.capabilities.some((capability) => capability.key === "wallet" && capability.enabled),
    ).length,
  };
}

export function findOrganizationWorkspaceItem(
  session: CurrentSession,
  organizationId: number,
): OrganizationWorkspaceItem | null {
  return buildOrganizationWorkspace(session).organizations.find((organization) => organization.id === organizationId) || null;
}

export function organizationMatchesCapability(
  organization: OrganizationWorkspaceItem,
  capability: OrganizationCapabilityKey | "all" | "delegated",
) {
  if (capability === "all") {
    return true;
  }

  if (capability === "delegated") {
    return organization.delegatedPermissionCount > 0;
  }

  return organization.capabilities.some((item) => item.key === capability && item.enabled);
}

export function filterOrganizationWorkspace(
  organizations: OrganizationWorkspaceItem[],
  filter: OrganizationFilter,
) {
  const normalizedSearch = filter.search.trim().toLocaleLowerCase();

  return organizations.filter((organization) => {
    const matchesCapability = organizationMatchesCapability(organization, filter.capability);
    const matchesSearch =
      normalizedSearch.length === 0 ||
      organization.name.toLocaleLowerCase().includes(normalizedSearch) ||
      organization.roles.some((role) => role.toLocaleLowerCase().includes(normalizedSearch)) ||
      organization.effectivePermissions.some((permission) =>
        permission.toLocaleLowerCase().includes(normalizedSearch),
      );

    return matchesCapability && matchesSearch;
  });
}

export function enabledOrganizationCapabilities(organization: OrganizationWorkspaceItem) {
  return organization.capabilities.filter((capability) => capability.enabled);
}

export function missingOrganizationPermissions(capability: OrganizationCapability) {
  return capability.permissions;
}

function buildOrganizationWorkspaceItem(organization: OrganizationSessionScope): OrganizationWorkspaceItem {
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

function hasOrganizationScopeSignal(organization: OrganizationSessionScope) {
  return (
    organization.roles.length > 0 ||
    organization.direct_permissions.length > 0 ||
    organization.delegated_permissions.length > 0 ||
    organization.effective_permissions.length > 0
  );
}
