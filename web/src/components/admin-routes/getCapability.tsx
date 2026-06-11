"use client";

import { platformCapabilityDefinitions, type PlatformAdminWorkspace, type PlatformCapability, type PlatformCapabilityKey } from "@/lib/admin";

export function getCapability(workspace: PlatformAdminWorkspace, key: PlatformCapabilityKey): PlatformCapability {
  const capability = workspace.capabilities.find((item) => item.key === key);
  if (capability) {
    return capability;
  }

  const definition = platformCapabilityDefinitions.find((item) => item.key === key);
  return {
    enabled: false,
    key,
    label: definition?.label || key,
    permissions: definition?.permissions || [],
  };
}
