import { type PlatformCapability } from "./PlatformCapability";

export type PlatformAdminWorkspace = {
  capabilities: PlatformCapability[];
  delegatedPermissionCount: number;
  directPermissionCount: number;
  effectivePermissionCount: number;
  effectivePermissions: string[];
  roles: string[];
};
