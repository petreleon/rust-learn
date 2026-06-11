import { type CurrentSession } from "@/lib/session";
import { platformCapabilityDefinitions } from "./platformCapabilityDefinitions";
import { type PlatformAdminWorkspace } from "./PlatformAdminWorkspace";

export function buildPlatformAdminWorkspace(session: CurrentSession): PlatformAdminWorkspace {
  return {
    capabilities: platformCapabilityDefinitions.map((capability) => ({
      enabled: capability.permissions.some((permission) => session.platform.effective_permissions.includes(permission)),
      key: capability.key,
      label: capability.label,
      permissions: capability.permissions,
    })),
    delegatedPermissionCount: session.platform.delegated_permissions.length,
    directPermissionCount: session.platform.direct_permissions.length,
    effectivePermissionCount: session.platform.effective_permissions.length,
    effectivePermissions: [...session.platform.effective_permissions],
    roles: [...session.platform.roles],
  };
}
