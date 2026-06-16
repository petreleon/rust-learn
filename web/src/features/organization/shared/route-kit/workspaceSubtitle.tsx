"use client";

import { enabledOrganizationCapabilities, type OrganizationWorkspaceItem } from "@/lib/organization";

export function workspaceSubtitle(organization: OrganizationWorkspaceItem) {
  const enabledCount = enabledOrganizationCapabilities(organization).length;
  if (enabledCount > 0) {
    return `${enabledCount} action ${enabledCount === 1 ? "capability" : "capabilities"} available`;
  }

  if (organization.delegatedPermissionCount > 0) {
    return "Delegated organization access";
  }

  return "Organization membership visible";
}
