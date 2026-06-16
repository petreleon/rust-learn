import { platformPermissionEnabled } from "@/lib/admin/platformPermissionEnabled";
import { type PlatformAdminWorkspace } from "@/lib/admin/PlatformAdminWorkspace";
import { type PlatformCapability } from "@/lib/admin/PlatformCapability";

export const emptyRewardPolicyWorkspace: PlatformAdminWorkspace = {
  capabilities: [],
  delegatedPermissionCount: 0,
  directPermissionCount: 0,
  effectivePermissionCount: 0,
  effectivePermissions: [],
  roles: [],
};

export function canManageRewardPolicies(workspace: PlatformAdminWorkspace) {
  return platformPermissionEnabled(workspace, "SET_REWARD_POLICY");
}

export function findRewardPolicyCapability(workspace: PlatformAdminWorkspace): PlatformCapability {
  return (
    workspace.capabilities.find((item) => item.key === "reward_policies") ?? {
      enabled: false,
      key: "reward_policies",
      label: "Reward policies",
      permissions: ["SET_REWARD_POLICY"],
    }
  );
}
