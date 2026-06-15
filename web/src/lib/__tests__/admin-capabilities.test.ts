import { describe, expect, it } from "vitest";
import { platformPermissionEnabled, type PlatformAdminWorkspace } from "@/lib/admin";

function workspace({
  capabilityEnabled = true,
  capabilityPermissions,
  effectivePermissions,
}: {
  capabilityEnabled?: boolean;
  capabilityPermissions: string[];
  effectivePermissions: string[];
}): PlatformAdminWorkspace {
  return {
    capabilities: [
      {
        enabled: capabilityEnabled,
        key: "delegations",
        label: "Delegations",
        permissions: capabilityPermissions,
      },
    ],
    delegatedPermissionCount: 0,
    directPermissionCount: effectivePermissions.length,
    effectivePermissionCount: effectivePermissions.length,
    effectivePermissions,
    roles: effectivePermissions.length ? ["platform_admin"] : [],
  };
}

describe("platformPermissionEnabled", () => {
  it("requires the backend capability to declare the permission", () => {
    const adminWorkspace = workspace({
      capabilityPermissions: ["MANAGE_ROLE_PERMISSIONS"],
      effectivePermissions: ["MANAGE_ROLE_PERMISSIONS", "DELEGATE_REWARD_APPROVAL"],
    });

    expect(platformPermissionEnabled(adminWorkspace, "MANAGE_ROLE_PERMISSIONS")).toBe(true);
    expect(platformPermissionEnabled(adminWorkspace, "DELEGATE_REWARD_APPROVAL")).toBe(false);
  });

  it("requires the capability to be enabled by the current session", () => {
    const adminWorkspace = workspace({
      capabilityEnabled: false,
      capabilityPermissions: ["DELEGATE_REWARD_APPROVAL"],
      effectivePermissions: ["DELEGATE_REWARD_APPROVAL"],
    });

    expect(platformPermissionEnabled(adminWorkspace, "DELEGATE_REWARD_APPROVAL")).toBe(false);
  });
});
