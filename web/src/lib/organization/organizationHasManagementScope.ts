import { type OrganizationWorkspaceItem } from "./OrganizationWorkspaceItem";

export function organizationHasManagementScope(organization: OrganizationWorkspaceItem) {
  return organization.capabilities.some(
    (capability) =>
      capability.enabled &&
      ["member_management", "teacher_applications", "settings"].includes(capability.key),
  );
}
