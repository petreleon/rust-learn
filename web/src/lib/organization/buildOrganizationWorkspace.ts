import { type CurrentSession } from "@/lib/session";
import { type OrganizationWorkspaceSummary } from "./OrganizationWorkspaceSummary";
import { buildOrganizationWorkspaceItem } from "./buildOrganizationWorkspaceItem";
import { hasOrganizationScopeSignal } from "./hasOrganizationScopeSignal";
import { organizationHasManagementScope } from "./organizationHasManagementScope";

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
    managementScopeCount: organizations.filter((organization) => organizationHasManagementScope(organization)).length,
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
