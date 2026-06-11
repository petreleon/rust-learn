import { type OrganizationCapability } from "./OrganizationCapability";

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
