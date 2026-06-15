import { type CurrentSession, type SessionCapability } from "@/lib/session";
import { type PlatformAdminWorkspace } from "./PlatformAdminWorkspace";
import { type PlatformCapabilityKey } from "./PlatformCapabilityKey";

const platformCapabilityKeys = new Set<string>([
  "summary",
  "kyc_reviews",
  "teacher_applications",
  "reward_amount_review",
  "fraud_blocks",
  "delegations",
  "exports",
  "wallets",
  "system",
]);

function hasPlatformCapabilityKey(
  capability: SessionCapability,
): capability is SessionCapability & { key: PlatformCapabilityKey } {
  return platformCapabilityKeys.has(capability.key);
}

export function buildPlatformAdminWorkspace(session: CurrentSession): PlatformAdminWorkspace {
  return {
    capabilities: session.platform.capabilities
      .filter(hasPlatformCapabilityKey)
      .map((capability) => ({ ...capability, key: capability.key })),
    delegatedPermissionCount: session.platform.delegated_permissions.length,
    directPermissionCount: session.platform.direct_permissions.length,
    effectivePermissionCount: session.platform.effective_permissions.length,
    effectivePermissions: [...session.platform.effective_permissions],
    roles: [...session.platform.roles],
  };
}
