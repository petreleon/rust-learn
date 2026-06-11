"use client";

import { platformCapabilityDefinitions, type PlatformAdminWorkspace } from "@/lib/admin";

export const emptyWorkspace: PlatformAdminWorkspace = {
  capabilities: platformCapabilityDefinitions.map((capability) => ({
    ...capability,
    enabled: false,
  })),
  delegatedPermissionCount: 0,
  directPermissionCount: 0,
  effectivePermissionCount: 0,
  effectivePermissions: [],
  roles: [],
};
