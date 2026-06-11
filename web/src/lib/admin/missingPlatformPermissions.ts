import { type PlatformCapability } from "./PlatformCapability";

export function missingPlatformPermissions(capability: PlatformCapability) {
  return capability.permissions;
}
