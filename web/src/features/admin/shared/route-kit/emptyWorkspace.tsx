"use client";

import { type PlatformAdminWorkspace } from "@/lib/admin";

export const emptyWorkspace: PlatformAdminWorkspace = {
  capabilities: [],
  delegatedPermissionCount: 0,
  directPermissionCount: 0,
  effectivePermissionCount: 0,
  effectivePermissions: [],
  roles: [],
};
