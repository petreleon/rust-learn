"use client";

import { platformCapabilityEnabled, type PlatformAdminWorkspace, type PlatformCapabilityKey } from "@/lib/admin";

export function hasPlatformCapability(workspace: PlatformAdminWorkspace, key: PlatformCapabilityKey) {
  return platformCapabilityEnabled(workspace, key);
}
