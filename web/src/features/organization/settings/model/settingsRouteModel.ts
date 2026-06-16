import { organizationPermissionEnabled } from "@/lib/organization/organizationPermissionEnabled";
import { type OrganizationCapability } from "@/lib/organization/OrganizationCapability";
import { type OrganizationWorkspaceItem } from "@/lib/organization/OrganizationWorkspaceItem";

export function parseOrganizationRouteId(organizationId: string) {
  const numericOrganizationId = Number.parseInt(organizationId, 10);

  return {
    invalidOrganizationId: !/^\d+$/.test(organizationId) || !Number.isFinite(numericOrganizationId),
    numericOrganizationId,
  };
}

export function canManageOrganizationSettings(organization: OrganizationWorkspaceItem | null) {
  return organizationPermissionEnabled(organization, "MANAGE_ORG_SETTINGS");
}

export function findSettingsCapability(organization: OrganizationWorkspaceItem | null): OrganizationCapability | undefined {
  return organization?.capabilities.find((capability) => capability.key === "settings");
}

export function organizationSettingsTitle(organization: OrganizationWorkspaceItem | null) {
  return organization?.name ? `${organization.name} settings` : "Organization settings";
}
