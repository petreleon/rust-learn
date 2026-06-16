"use client";

import { type PlatformAdminWorkspace, type PlatformCapability, type PlatformCapabilityKey } from "@/lib/admin";

export function getCapability(workspace: PlatformAdminWorkspace, key: PlatformCapabilityKey): PlatformCapability {
  const capability = workspace.capabilities.find((item) => item.key === key);
  if (capability) {
    return capability;
  }

  return {
    enabled: false,
    key,
    label: key,
    permissions: [],
  };
}
