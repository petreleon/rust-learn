import { platformCapabilityEnabled } from "@/lib/admin/platformCapabilityEnabled";
import { type PlatformAdminWorkspace } from "@/lib/admin/PlatformAdminWorkspace";
import { type PlatformCapability } from "@/lib/admin/PlatformCapability";
import { type PlatformCapabilityKey } from "@/lib/admin/PlatformCapabilityKey";

export const emptyPlatformWalletWorkspace: PlatformAdminWorkspace = {
  capabilities: [],
  delegatedPermissionCount: 0,
  directPermissionCount: 0,
  effectivePermissionCount: 0,
  effectivePermissions: [],
  roles: [],
};

export function canViewWalletAudit(workspace: PlatformAdminWorkspace) {
  return platformCapabilityEnabled(workspace, "wallets");
}

export function canExportWalletCredits(workspace: PlatformAdminWorkspace) {
  return platformCapabilityEnabled(workspace, "exports");
}

export function canSetDepositTax(workspace: PlatformAdminWorkspace) {
  return workspace.effectivePermissions.includes("SET_DEPOSIT_TAX");
}

export function canSetRetireTax(workspace: PlatformAdminWorkspace) {
  return workspace.effectivePermissions.includes("SET_RETIRE_TAX");
}

export function canManageTokenTaxes(workspace: PlatformAdminWorkspace) {
  return canSetDepositTax(workspace) || canSetRetireTax(workspace);
}

export function canViewBurnLeaderboard(workspace: PlatformAdminWorkspace) {
  return workspace.effectivePermissions.includes("VIEW_BURN_LEADERBOARD");
}

export function findPlatformCapability(
  workspace: PlatformAdminWorkspace,
  key: PlatformCapabilityKey,
): PlatformCapability {
  return (
    workspace.capabilities.find((item) => item.key === key) ?? {
      enabled: false,
      key,
      label: key,
      permissions: [],
    }
  );
}
