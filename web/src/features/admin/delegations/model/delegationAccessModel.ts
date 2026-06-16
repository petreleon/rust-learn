import { platformPermissionEnabled } from "@/lib/admin/platformPermissionEnabled";
import { type PlatformAdminWorkspace } from "@/lib/admin/PlatformAdminWorkspace";
import { type PlatformCapability } from "@/lib/admin/PlatformCapability";

export const emptyDelegationWorkspace: PlatformAdminWorkspace = {
  capabilities: [],
  delegatedPermissionCount: 0,
  directPermissionCount: 0,
  effectivePermissionCount: 0,
  effectivePermissions: [],
  roles: [],
};

export function canViewDelegations(workspace: PlatformAdminWorkspace) {
  return hasAnyPlatformPermission(workspace, ["VIEW_ROLE_ASSIGNMENTS", "MANAGE_ROLE_PERMISSIONS"]);
}

export function canGrantDelegations(workspace: PlatformAdminWorkspace) {
  return hasAnyPlatformPermission(workspace, ["MANAGE_ROLE_PERMISSIONS", "DELEGATE_REWARD_APPROVAL"]);
}

export function canRevokeDelegations(workspace: PlatformAdminWorkspace) {
  return platformPermissionEnabled(workspace, "MANAGE_ROLE_PERMISSIONS");
}

export function findDelegationCapability(workspace: PlatformAdminWorkspace): PlatformCapability {
  return (
    workspace.capabilities.find((item) => item.key === "delegations") ?? {
      enabled: false,
      key: "delegations",
      label: "Delegations",
      permissions: ["VIEW_ROLE_ASSIGNMENTS", "MANAGE_ROLE_PERMISSIONS"],
    }
  );
}

function hasAnyPlatformPermission(workspace: PlatformAdminWorkspace, permissions: string[]) {
  return permissions.some((permission) => platformPermissionEnabled(workspace, permission));
}
