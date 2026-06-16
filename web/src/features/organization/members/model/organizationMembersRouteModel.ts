import { type OrganizationCapability } from "@/lib/organization/OrganizationCapability";
import { type OrganizationWorkspaceItem } from "@/lib/organization/OrganizationWorkspaceItem";

export function parseOrganizationRouteId(organizationId: string) {
  const numericOrganizationId = Number.parseInt(organizationId, 10);

  return {
    invalidOrganizationId: !/^\d+$/.test(organizationId) || !Number.isFinite(numericOrganizationId),
    numericOrganizationId,
  };
}

export function findMembersCapability(organization: OrganizationWorkspaceItem | null): OrganizationCapability | undefined {
  return organization?.capabilities.find((capability) => capability.key === "members");
}

export function membersCapabilityEnabled(capability?: OrganizationCapability) {
  return Boolean(capability?.enabled);
}

export function organizationMembersTitle(organization: OrganizationWorkspaceItem | null) {
  return organization?.name ? `${organization.name} members` : "Organization members";
}
