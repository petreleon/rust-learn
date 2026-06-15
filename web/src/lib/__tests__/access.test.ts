import { describe, it, expect } from "vitest";
import {
  accessSummary,
  hasAnyScopePermission,
  hasTeacherApplicationAccess,
  hasTeacherCourseAccess,
  hasOrganizationAccess,
  hasPlatformAdminAccess,
  countDelegatedPermissions,
} from "@/lib/access";
import type {
  CurrentSession,
  PlatformSessionScope,
  CourseSessionScope,
  OrganizationSessionScope,
  DelegatedPermissionSession,
  SessionCapability,
} from "@/lib/session";

function capability(key: string, enabled = true): SessionCapability {
  return { enabled, key, label: key, permissions: [] };
}

function makePlatformScope(permissions: string[], capabilities: SessionCapability[] = []): PlatformSessionScope {
  return {
    roles: [],
    direct_permissions: permissions,
    delegated_permissions: [],
    effective_permissions: permissions,
    capabilities,
  };
}

function makeCourseScope(permissions: string[], capabilities: SessionCapability[] = []): CourseSessionScope {
  return {
    id: 1,
    title: "Course 1",
    lifecycle_status: "published",
    roles: [],
    direct_permissions: permissions,
    delegated_permissions: [],
    effective_permissions: permissions,
    capabilities,
  };
}

function makeOrgScope(permissions: string[], capabilities: SessionCapability[] = []): OrganizationSessionScope {
  return {
    id: 1,
    name: "Org 1",
    roles: [],
    direct_permissions: permissions,
    delegated_permissions: [],
    effective_permissions: permissions,
    capabilities,
  };
}

function makeSession(overrides: Partial<CurrentSession> = {}): CurrentSession {
  return {
    access: {
      learner: true,
      teacher: false,
      teacher_application: false,
      organization: false,
      platform_admin: false,
    },
    user: { id: 1, name: "Test", email: "test@e.com", email_verified: true, kyc_verified: false },
    platform: makePlatformScope([]),
    organizations: [],
    courses: [],
    delegated_permissions: [],
    ...overrides,
  };
}

describe("accessSummary", () => {
  it("returns all false for null session", () => {
    const s = accessSummary(null);
    expect(s.learner).toBe(false);
    expect(s.teacher).toBe(false);
    expect(s.organization).toBe(false);
    expect(s.platformAdmin).toBe(false);
  });

  it("learner is always true when session exists", () => {
    expect(accessSummary(makeSession()).learner).toBe(true);
  });

  it("teacher is true when backend access says teacher", () => {
    const s = makeSession({ access: { ...makeSession().access, teacher: true } });
    expect(accessSummary(s).teacher).toBe(true);
  });

  it("teacher course access uses backend course capabilities", () => {
    const course = makeCourseScope(["MANAGE_COURSE_SETTINGS"], [capability("teaching")]);
    expect(hasTeacherCourseAccess(course)).toBe(true);
  });

  it("teacher application access uses backend access", () => {
    const s = makeSession({ access: { ...makeSession().access, teacher_application: true } });
    expect(hasTeacherApplicationAccess(s)).toBe(true);
  });

  it("teacher is false when backend access denies it", () => {
    expect(accessSummary(makeSession()).teacher).toBe(false);
  });
});

describe("hasAnyScopePermission", () => {
  it("matches permissions from a set", () => {
    const scope = makePlatformScope(["VIEW_REWARD_AUDIT", "EXPORT_DATA"]);
    const perms = new Set(["VIEW_REWARD_AUDIT"]);
    expect(hasAnyScopePermission(scope, perms)).toBe(true);
  });

  it("returns false for no match", () => {
    expect(hasAnyScopePermission(makePlatformScope([]), new Set(["X"]))).toBe(false);
  });
});

describe("hasOrganizationAccess", () => {
  it("true when org has roles", () => {
    const org = { ...makeOrgScope([]), roles: ["ADMIN"] };
    expect(hasOrganizationAccess(org)).toBe(true);
  });

  it("true when org has direct permissions", () => {
    const org = { ...makeOrgScope([]), direct_permissions: ["VIEW_ORGANIZATION"], effective_permissions: ["VIEW_ORGANIZATION"] };
    expect(hasOrganizationAccess(org)).toBe(true);
  });

  it("true when org has backend capabilities", () => {
    expect(hasOrganizationAccess(makeOrgScope([], [capability("reports")]))).toBe(true);
  });

  it("false when org has nothing", () => {
    expect(hasOrganizationAccess(makeOrgScope([]))).toBe(false);
  });
});

describe("countDelegatedPermissions", () => {
  it("returns 0 for null", () => {
    expect(countDelegatedPermissions(null)).toBe(0);
  });

  it("counts delegated permissions", () => {
    const s = makeSession({
      delegated_permissions: [{ id: 1 } as DelegatedPermissionSession, { id: 2 } as DelegatedPermissionSession],
    });
    expect(countDelegatedPermissions(s)).toBe(2);
  });
});

describe("platform admin access", () => {
  it("hasPlatformAdminAccess follows backend access", () => {
    const s = makeSession({ access: { ...makeSession().access, platform_admin: true } });
    expect(hasPlatformAdminAccess(s)).toBe(true);
  });

  it("hasPlatformAdminAccess false without permission", () => {
    expect(hasPlatformAdminAccess(makeSession())).toBe(false);
  });
});
