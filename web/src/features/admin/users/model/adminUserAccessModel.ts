import { platformCapabilityEnabled, platformPermissionEnabled, type PlatformAdminWorkspace } from "@/lib/admin";

export const emptyAdminUsersWorkspace: PlatformAdminWorkspace = {
  capabilities: [],
  delegatedPermissionCount: 0,
  directPermissionCount: 0,
  effectivePermissionCount: 0,
  effectivePermissions: [],
  roles: [],
};

export function canViewAdminUsers(workspace: PlatformAdminWorkspace) {
  return platformPermissionEnabled(workspace, "VIEW_USER");
}

export function canAssignAdminUserRoles(workspace: PlatformAdminWorkspace) {
  return platformPermissionEnabled(workspace, "ASSIGN_ROLES_TO_USER");
}

export function canViewPlatformRoleCatalog(workspace: PlatformAdminWorkspace) {
  return platformPermissionEnabled(workspace, "VIEW_ROLE_ASSIGNMENTS");
}

export function findUsersCapability(workspace: PlatformAdminWorkspace) {
  return (
    workspace.capabilities.find((capability) => capability.key === "users") ?? {
      enabled: platformCapabilityEnabled(workspace, "users"),
      key: "users" as const,
      label: "User management",
      permissions: ["VIEW_USER", "ASSIGN_ROLES_TO_USER"],
    }
  );
}
