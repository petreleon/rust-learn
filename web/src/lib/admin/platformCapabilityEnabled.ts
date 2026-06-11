import { type PlatformAdminWorkspace } from "./PlatformAdminWorkspace";
import { type PlatformCapabilityKey } from "./PlatformCapabilityKey";

export function platformCapabilityEnabled(workspace: PlatformAdminWorkspace, key: PlatformCapabilityKey) {
  return workspace.capabilities.some((capability) => capability.key === key && capability.enabled);
}
