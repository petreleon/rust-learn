import { platformPermissionEnabled } from "@/lib/admin/platformPermissionEnabled";
import { type PlatformAdminWorkspace } from "@/lib/admin/PlatformAdminWorkspace";
import { type PlatformCapability } from "@/lib/admin/PlatformCapability";

export const emptyFraudBlockWorkspace: PlatformAdminWorkspace = {
  capabilities: [],
  delegatedPermissionCount: 0,
  directPermissionCount: 0,
  effectivePermissionCount: 0,
  effectivePermissions: [],
  roles: [],
};

export function canViewFraudBlocks(workspace: PlatformAdminWorkspace) {
  return hasAnyPlatformPermission(workspace, ["VIEW_REWARD_AUDIT", "MANAGE_REWARD_FRAUD_BLOCKS"]);
}

export function canCreateFraudBlocks(workspace: PlatformAdminWorkspace) {
  return hasAnyPlatformPermission(workspace, [
    "MANAGE_REWARD_FRAUD_BLOCKS",
    "BLOCK_REWARD_TEACHER",
    "BLOCK_REWARD_ORGANIZATION",
  ]);
}

export function canListRewardPoliciesForBlocks(workspace: PlatformAdminWorkspace) {
  return workspace.effectivePermissions.includes("SET_REWARD_POLICY");
}

export function canRevokeFraudBlocks(workspace: PlatformAdminWorkspace) {
  return platformPermissionEnabled(workspace, "MANAGE_REWARD_FRAUD_BLOCKS");
}

export function findFraudBlockCapability(workspace: PlatformAdminWorkspace): PlatformCapability {
  return (
    workspace.capabilities.find((item) => item.key === "fraud_blocks") ?? {
      enabled: false,
      key: "fraud_blocks",
      label: "Fraud blocks",
      permissions: ["VIEW_REWARD_AUDIT", "MANAGE_REWARD_FRAUD_BLOCKS"],
    }
  );
}

function hasAnyPlatformPermission(workspace: PlatformAdminWorkspace, permissions: string[]) {
  return permissions.some((permission) => platformPermissionEnabled(workspace, permission));
}
