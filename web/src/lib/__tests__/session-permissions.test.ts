import { describe, expect, it } from "vitest";
import {
  countSessionCapabilityPermissions,
  sessionCapabilityPermissionGroups,
  sessionPermissionEnabled,
  sessionScopePermissionEnabled,
} from "@/lib/session";
import type { CurrentSession, PlatformSessionScope, SessionCapability } from "@/lib/session";

function capability(
  key: string,
  permissions: string[],
  enabled = true,
): SessionCapability {
  return { enabled, key, label: key, permissions };
}

function scope(
  effectivePermissions: string[],
  capabilities: SessionCapability[] = [],
): PlatformSessionScope {
  return {
    capabilities,
    delegated_permissions: [],
    direct_permissions: effectivePermissions,
    effective_permissions: effectivePermissions,
    roles: [],
  };
}

function session(overrides: Partial<CurrentSession> = {}): CurrentSession {
  return {
    access: {
      learner: true,
      organization: false,
      platform_admin: false,
      teacher: false,
      teacher_application: false,
    },
    courses: [],
    delegated_permissions: [],
    organizations: [],
    platform: scope([]),
    user: {
      email: "t@example.com",
      email_verified: true,
      id: 1,
      kyc_verified: false,
      name: "T",
    },
    ...overrides,
  };
}

describe("sessionScopePermissionEnabled", () => {
  it("requires both an effective permission and an enabled capability declaration", () => {
    expect(
      sessionScopePermissionEnabled(
        scope(["VIEW_REPORT"], [capability("summary", ["VIEW_REPORT"])]),
        "VIEW_REPORT",
      ),
    ).toBe(true);
    expect(
      sessionScopePermissionEnabled(
        scope(["VIEW_REPORT"], [capability("summary", ["VIEW_REPORT"], false)]),
        "VIEW_REPORT",
      ),
    ).toBe(false);
    expect(
      sessionScopePermissionEnabled(scope(["VIEW_REPORT"], []), "VIEW_REPORT"),
    ).toBe(false);
  });
});

describe("sessionPermissionEnabled", () => {
  it("finds capability-backed permissions across platform, organization, and course scopes", () => {
    const current = session({
      courses: [
        {
          ...scope(["VIEW_COURSE_REWARD_STATUS"], [
            capability("reward_status", ["VIEW_COURSE_REWARD_STATUS"]),
          ]),
          id: 2,
          lifecycle_status: "published",
          title: "Rust",
        },
      ],
      organizations: [
        {
          ...scope(["GENERATE_REPORT"], [capability("reports", ["GENERATE_REPORT"])]),
          id: 3,
          name: "Org",
        },
      ],
      platform: scope(["VIEW_REPORT"], [capability("summary", ["VIEW_REPORT"])]),
    });

    expect(sessionPermissionEnabled(current, "VIEW_REPORT")).toBe(true);
    expect(sessionPermissionEnabled(current, "GENERATE_REPORT")).toBe(true);
    expect(sessionPermissionEnabled(current, "VIEW_COURSE_REWARD_STATUS")).toBe(true);
  });

  it("uses backend teacher application access for submission permission", () => {
    const current = session({
      access: {
        learner: true,
        organization: false,
        platform_admin: false,
        teacher: true,
        teacher_application: true,
      },
    });

    expect(sessionPermissionEnabled(current, "SUBMIT_TEACHER_APPLICATION")).toBe(true);
  });
});

describe("sessionCapabilityPermissionGroups", () => {
  it("returns sorted active permissions grouped by backend session scope", () => {
    const current = session({
      access: {
        learner: true,
        organization: false,
        platform_admin: true,
        teacher: true,
        teacher_application: true,
      },
      platform: scope(["VIEW_REPORT", "EXPORT_DATA"], [
        capability("exports", ["EXPORT_DATA"]),
        capability("summary", ["VIEW_REPORT"]),
      ]),
    });

    expect(sessionCapabilityPermissionGroups(current)[0].permissions).toEqual([
      "EXPORT_DATA",
      "SUBMIT_TEACHER_APPLICATION",
      "VIEW_REPORT",
    ]);
    expect(countSessionCapabilityPermissions(current)).toBe(3);
  });
});
