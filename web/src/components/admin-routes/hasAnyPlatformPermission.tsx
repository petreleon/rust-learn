"use client";

import { type PlatformAdminWorkspace } from "@/lib/admin";
import { hasPlatformPermission } from "./hasPlatformPermission";

export function hasAnyPlatformPermission(workspace: PlatformAdminWorkspace, permissions: string[]) {
  return permissions.some((permission) => hasPlatformPermission(workspace, permission));
}
