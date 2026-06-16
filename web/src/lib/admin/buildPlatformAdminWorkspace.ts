import { type CurrentSession, type SessionCapability } from "@/lib/session";
import { type PlatformAdminWorkspace } from "./PlatformAdminWorkspace";
import { type PlatformCapabilityKey } from "./PlatformCapabilityKey";

const platformCapabilityKeys = new Set<string>([
  "summary",
  "users",
  "kyc_reviews",
  "teacher_applications",
  "reward_amount_review",
  "reward_policies",
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
  const platform = session.platform;
  const capabilities = platform.capabilities?.length
    ? platform.capabilities
    : legacyPlatformCapabilities(platform.effective_permissions);

  return {
    capabilities: capabilities
      .filter(hasPlatformCapabilityKey)
      .map((capability) => ({ ...capability, key: capability.key })),
    delegatedPermissionCount: platform.delegated_permissions.length,
    directPermissionCount: platform.direct_permissions.length,
    effectivePermissionCount: platform.effective_permissions.length,
    effectivePermissions: [...platform.effective_permissions],
    roles: [...platform.roles],
  };
}

function legacyPlatformCapabilities(permissions: string[]): SessionCapability[] {
  const enabled = new Set(permissions);
  return [
    capability("summary", "Platform summary", ["VIEW_REPORT"], enabled),
    capability("users", "User management", ["VIEW_USER", "ASSIGN_ROLES_TO_USER", "VIEW_ROLE_ASSIGNMENTS"], enabled),
    capability("kyc_reviews", "KYC review", ["REVIEW_KYC_SUBMISSIONS"], enabled),
    capability("teacher_applications", "Teacher applications", ["REVIEW_TEACHER_APPLICATIONS"], enabled),
    capability("reward_amount_review", "Reward amount review", ["APPROVE_REWARD_AMOUNT"], enabled),
    capability("reward_policies", "Reward policies", ["SET_REWARD_POLICY"], enabled),
    capability("fraud_blocks", "Fraud blocks", ["MANAGE_REWARD_FRAUD_BLOCKS"], enabled),
    capability("delegations", "Delegations", ["VIEW_ROLE_ASSIGNMENTS", "MANAGE_ROLE_PERMISSIONS"], enabled),
    capability("exports", "Exports", ["EXPORT_DATA"], enabled),
    capability("wallets", "Wallets", ["VIEW_TRANSACTIONS"], enabled),
    capability("system", "System", ["VIEW_TRANSACTIONS"], enabled),
  ];
}

function capability(
  key: PlatformCapabilityKey,
  label: string,
  permissions: string[],
  enabled: Set<string>,
): SessionCapability {
  return {
    enabled: permissions.some((permission) => enabled.has(permission)),
    key,
    label,
    permissions,
  };
}
