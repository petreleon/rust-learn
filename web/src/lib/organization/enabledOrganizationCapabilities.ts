import { type OrganizationWorkspaceItem } from "./OrganizationWorkspaceItem";

export function enabledOrganizationCapabilities(organization: OrganizationWorkspaceItem) {
  return organization.capabilities.filter((capability) => capability.enabled);
}
