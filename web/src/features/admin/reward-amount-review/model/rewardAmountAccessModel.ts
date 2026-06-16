import { platformPermissionEnabled } from "@/lib/admin/platformPermissionEnabled";
import { type PlatformAdminWorkspace } from "@/lib/admin/PlatformAdminWorkspace";
import { type PlatformCapability } from "@/lib/admin/PlatformCapability";

export const emptyRewardAmountWorkspace: PlatformAdminWorkspace = {
  capabilities: [],
  delegatedPermissionCount: 0,
  directPermissionCount: 0,
  effectivePermissionCount: 0,
  effectivePermissions: [],
  roles: [],
};

export function canViewRewardAmountReview(workspace: PlatformAdminWorkspace) {
  return platformPermissionEnabled(workspace, "VIEW_REWARD_AUDIT");
}

export function canApproveRewardAmount(workspace: PlatformAdminWorkspace) {
  return platformPermissionEnabled(workspace, "APPROVE_REWARD_AMOUNT");
}

export function findRewardAmountCapability(workspace: PlatformAdminWorkspace): PlatformCapability {
  return (
    workspace.capabilities.find((item) => item.key === "reward_amount_review") ?? {
      enabled: false,
      key: "reward_amount_review",
      label: "Reward amount review",
      permissions: ["VIEW_REWARD_AUDIT"],
    }
  );
}
