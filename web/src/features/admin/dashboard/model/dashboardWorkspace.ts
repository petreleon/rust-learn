import { platformCapabilityEnabled } from "@/lib/admin/platformCapabilityEnabled";
import { platformPermissionEnabled } from "@/lib/admin/platformPermissionEnabled";
import { type PlatformAdminWorkspace } from "@/lib/admin/PlatformAdminWorkspace";

export const emptyPlatformAdminWorkspace: PlatformAdminWorkspace = {
  capabilities: [],
  delegatedPermissionCount: 0,
  directPermissionCount: 0,
  effectivePermissionCount: 0,
  effectivePermissions: [],
  roles: [],
};

export function canViewPlatformSummary(workspace: PlatformAdminWorkspace) {
  return platformCapabilityEnabled(workspace, "summary");
}

export function canViewRewardAudit(workspace: PlatformAdminWorkspace) {
  return platformPermissionEnabled(workspace, "VIEW_REWARD_AUDIT");
}

export function canApproveRewardAmount(workspace: PlatformAdminWorkspace) {
  return platformPermissionEnabled(workspace, "APPROVE_REWARD_AMOUNT");
}

export function canExportPlatformData(workspace: PlatformAdminWorkspace) {
  return platformCapabilityEnabled(workspace, "exports");
}

export function canManageRewardFraud(workspace: PlatformAdminWorkspace) {
  return platformCapabilityEnabled(workspace, "fraud_blocks");
}
