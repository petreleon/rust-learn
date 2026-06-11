"use client";

import { type PlatformAdminWorkspace } from "@/lib/admin";

export function hasPlatformPermission(workspace: PlatformAdminWorkspace, permission: string) {
  return workspace.effectivePermissions.includes(permission);
}
