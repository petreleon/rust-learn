"use client";

import { platformPermissionEnabled, type PlatformAdminWorkspace } from "@/lib/admin";

export function hasPlatformPermission(workspace: PlatformAdminWorkspace, permission: string) {
  return platformPermissionEnabled(workspace, permission);
}
