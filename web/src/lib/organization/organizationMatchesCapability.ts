import { type OrganizationCapabilityKey } from "./OrganizationCapabilityKey";
import { type OrganizationWorkspaceItem } from "./OrganizationWorkspaceItem";

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
