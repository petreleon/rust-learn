import { describe, it, expect } from "vitest";
import {
  buildOrganizationWorkspace,
  organizationMatchesCapability,
  filterOrganizationWorkspace,
  enabledOrganizationCapabilities,
  organizationPermissionEnabled,
} from "@/lib/organization";
import type { OrganizationWorkspaceItem } from "@/lib/organization";
import type { CurrentSession } from "@/lib/session";

function makeSession(orgs: Array<{
  id: number; name: string; roles: string[]; permissions: string[]; capabilities?: string[];
}>) {
  return {
    access: { learner: true, teacher: false, teacher_application: false, organization: orgs.length > 0, platform_admin: false },
    user: { id: 1, name: "T", email: "t@e.com", email_verified: true, kyc_verified: false },
    platform: { roles: [], direct_permissions: [], delegated_permissions: [], effective_permissions: [], capabilities: [] },
    organizations: orgs.map((o) => ({
      id: o.id, name: o.name,
      roles: o.roles, direct_permissions: o.permissions,
      delegated_permissions: [], effective_permissions: o.permissions,
      capabilities: (o.capabilities ?? []).map((key) => ({ enabled: true, key, label: key, permissions: [] })),
    })),
    courses: [], delegated_permissions: [],
  } as CurrentSession;
}

describe("buildOrganizationWorkspace", () => {
  it("builds workspace from session organizations", () => {
    const session = makeSession([
      { id: 1, name: "ACME", roles: ["ADMIN"], permissions: ["VIEW_ORGANIZATION"] },
    ]);
    const workspace = buildOrganizationWorkspace(session);
    expect(workspace.total).toBeGreaterThanOrEqual(1);
    expect(workspace.organizations[0].name).toBe("ACME");
  });

  it("sorts organizations alphabetically", () => {
    const session = makeSession([
      { id: 1, name: "Z Corp", roles: [], permissions: ["VIEW_ORGANIZATION"] },
      { id: 2, name: "A Inc", roles: [], permissions: ["VIEW_ORGANIZATION"] },
    ]);
    const workspace = buildOrganizationWorkspace(session);
    expect(workspace.organizations[0].name).toBe("A Inc");
    expect(workspace.organizations[1].name).toBe("Z Corp");
  });
});

describe("organizationMatchesCapability", () => {
  // We use buildOrganizationWorkspace to get real OrganizationWorkspaceItems
  const org = buildOrganizationWorkspace(makeSession([
    { id: 1, name: "O", roles: [], permissions: ["VIEW_ORGANIZATION"], capabilities: ["members"] },
  ])).organizations[0];

  it('"all" always matches', () => {
    expect(organizationMatchesCapability(org, "all")).toBe(true);
  });

  it("matches on delegated count", () => {
    // org without delegated permissions
    expect(organizationMatchesCapability(org, "delegated")).toBe(false);
  });
});

describe("filterOrganizationWorkspace", () => {
  const orgs = buildOrganizationWorkspace(makeSession([
    { id: 1, name: "Alpha", roles: ["ADMIN"], permissions: ["VIEW_ORG_REWARD_REPORTS"], capabilities: ["reports"] },
    { id: 2, name: "Beta", roles: [], permissions: ["VIEW_ORGANIZATION"], capabilities: ["members"] },
  ])).organizations;

  it("filters by search name", () => {
    const result = filterOrganizationWorkspace(orgs, { search: "alpha", capability: "all" });
    expect(result).toHaveLength(1);
    expect(result[0].name).toBe("Alpha");
  });

  it("filters by search role", () => {
    const result = filterOrganizationWorkspace(orgs, { search: "admin", capability: "all" });
    expect(result).toHaveLength(1);
  });

  it("empty search returns all", () => {
    const result = filterOrganizationWorkspace(orgs, { search: "   ", capability: "all" });
    expect(result).toHaveLength(2);
  });
});

describe("enabledOrganizationCapabilities", () => {
  it("returns only enabled capabilities", () => {
    const org = buildOrganizationWorkspace(makeSession([
      { id: 1, name: "O", roles: [], permissions: ["VIEW_ORGANIZATION"], capabilities: ["members"] },
    ])).organizations[0];
    const caps = enabledOrganizationCapabilities(org);
    caps.forEach((c) => expect(c.enabled).toBe(true));
  });
});

function makeOrganizationItem(
  effectivePermissions: string[],
  capability: OrganizationWorkspaceItem["capabilities"][number],
): OrganizationWorkspaceItem {
  return {
    capabilities: [capability],
    delegatedPermissionCount: 0,
    directPermissionCount: effectivePermissions.length,
    effectivePermissionCount: effectivePermissions.length,
    effectivePermissions,
    id: 1,
    name: "O",
    permissionPreview: effectivePermissions,
    roles: [],
  };
}

describe("organizationPermissionEnabled", () => {
  it("requires an enabled capability declaring the permission", () => {
    const org = makeOrganizationItem(["MANAGE_ORG_SETTINGS"], {
      enabled: true,
      key: "settings",
      label: "Settings",
      permissions: ["MANAGE_ORG_SETTINGS"],
    });

    expect(organizationPermissionEnabled(org, "MANAGE_ORG_SETTINGS")).toBe(true);
  });

  it("rejects undeclared or disabled capability permissions", () => {
    const missingDeclaration = makeOrganizationItem(["MANAGE_ORG_SETTINGS"], {
      enabled: true,
      key: "settings",
      label: "Settings",
      permissions: [],
    });
    const disabledCapability = makeOrganizationItem(["MANAGE_ORG_SETTINGS"], {
      enabled: false,
      key: "settings",
      label: "Settings",
      permissions: ["MANAGE_ORG_SETTINGS"],
    });

    expect(organizationPermissionEnabled(missingDeclaration, "MANAGE_ORG_SETTINGS")).toBe(false);
    expect(organizationPermissionEnabled(disabledCapability, "MANAGE_ORG_SETTINGS")).toBe(false);
    expect(organizationPermissionEnabled(null, "MANAGE_ORG_SETTINGS")).toBe(false);
  });
});
