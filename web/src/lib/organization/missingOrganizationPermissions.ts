import { type OrganizationCapability } from "./OrganizationCapability";

export function missingOrganizationPermissions(capability: OrganizationCapability) {
  return capability.permissions;
}
