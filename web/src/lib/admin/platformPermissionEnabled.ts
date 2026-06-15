import { type PlatformAdminWorkspace } from "./PlatformAdminWorkspace";

export function platformPermissionEnabled(workspace: PlatformAdminWorkspace, permission: string) {
  return (
    workspace.effectivePermissions.includes(permission) &&
    workspace.capabilities.some(
      (capability) => capability.enabled && capability.permissions.includes(permission),
    )
  );
}
