"use client";

import { platformCapabilityEnabled, type PlatformAdminWorkspace, type PlatformCapabilityKey } from "@/lib/admin";
import { getCapability } from "./getCapability";

export function capabilityLabel(workspace: PlatformAdminWorkspace, key: PlatformCapabilityKey) {
  return platformCapabilityEnabled(workspace, key) ? "Available" : `Missing ${getCapability(workspace, key).permissions[0]}`;
}
