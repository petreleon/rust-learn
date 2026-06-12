import { describe, it, expect } from "vitest";
import { closeOtherProductShellMenus } from "@/components/product-shell/closeOtherProductShellMenus";

// These test the pure logic extracted from product-shell.tsx helper functions.

describe("delegationScopeLabel", () => {
  function delegationScopeLabel(d: { organization_name?: string | null; course_title?: string | null; scope_type?: string }) {
    return d.organization_name || d.course_title || d.scope_type || "unknown";
  }

  it("prefers organization name", () => expect(delegationScopeLabel({ organization_name: "ACME" })).toBe("ACME"));
  it("falls back to course title", () => expect(delegationScopeLabel({ course_title: "Rust 101" })).toBe("Rust 101"));
  it("falls back to scope type", () => expect(delegationScopeLabel({ scope_type: "platform" })).toBe("platform"));
  it("handles empty", () => expect(delegationScopeLabel({})).toBe("unknown"));
});

describe("delegationExpiryLabel", () => {
  function delegationExpiryLabel(expiresAt: string | null) {
    if (!expiresAt) return "No expiry";
    return `Expires ${expiresAt.replace("T", " ").slice(0, 16)}`;
  }

  it("no expiry for null", () => expect(delegationExpiryLabel(null)).toBe("No expiry"));
  it("formats date", () => {
    const result = delegationExpiryLabel("2026-12-31T23:59:00Z");
    expect(result).toContain("Expires");
    expect(result).toContain("2026-12-31");
  });
});

describe("buildWorkspaceOptions", () => {
  type Session = {
    user: { id: number };
    organizations: Array<{ id: number; name: string }>;
    courses: Array<{ id: number; title: string }>;
  };

  function buildWorkspaceOptions(session?: Session | null) {
    if (!session) return [];
    const options = [{ label: "Personal workspace", value: `user:${session.user.id}` }];
    options.push(
      ...session.organizations.map((org) => ({ label: org.name, value: `organization:${org.id}` })),
    );
    options.push(
      ...session.courses.map((course) => ({ label: course.title, value: `course:${course.id}` })),
    );
    return options;
  }

  it("returns empty for null session", () => {
    expect(buildWorkspaceOptions(null)).toEqual([]);
  });

  it("includes personal workspace", () => {
    const opts = buildWorkspaceOptions({ user: { id: 1 }, organizations: [], courses: [] });
    expect(opts).toHaveLength(1);
    expect(opts[0]).toEqual({ label: "Personal workspace", value: "user:1" });
  });

  it("includes orgs and courses", () => {
    const opts = buildWorkspaceOptions({
      user: { id: 1 },
      organizations: [{ id: 2, name: "ACME" }],
      courses: [{ id: 3, title: "Rust 101" }],
    });
    expect(opts).toHaveLength(3);
    expect(opts[1]).toEqual({ label: "ACME", value: "organization:2" });
    expect(opts[2]).toEqual({ label: "Rust 101", value: "course:3" });
  });
});

describe("buildNavItems", () => {
  type NavItem = { key: string; href: string; label: string };
  type AccessSummary = { learner: boolean; teacher: boolean; organization: boolean; platformAdmin: boolean };

  function buildNavItems(access: AccessSummary | null): NavItem[] {
    const base = [
      { key: "session", href: "/session", label: "Workspace" },
      { key: "account", href: "/settings/account", label: "Account" },
    ];
    if (!access) return base;
    return [
      base[0],
      access.learner ? { key: "learn", href: "/learn", label: "Learn" } : null,
      access.teacher ? { key: "teach", href: "/teach", label: "Teach" } : null,
      access.organization ? { key: "organizations", href: "/organizations", label: "Organizations" } : null,
      access.platformAdmin ? { key: "admin", href: "/admin", label: "Admin" } : null,
      base[1],
    ].filter((item): item is NavItem => Boolean(item));
  }

  it("returns base items when no session", () => {
    const items = buildNavItems(null);
    expect(items).toHaveLength(2);
    expect(items.map((i) => i.key)).toEqual(["session", "account"]);
  });

  it("adds learner nav when access.learner", () => {
    const items = buildNavItems({ learner: true, teacher: false, organization: false, platformAdmin: false });
    expect(items.map((i) => i.key)).toContain("learn");
  });

  it("adds admin nav when platform admin", () => {
    const items = buildNavItems({ learner: true, teacher: true, organization: true, platformAdmin: true });
    expect(items.map((i) => i.key)).toEqual(["session", "learn", "teach", "organizations", "admin", "account"]);
  });
});

describe("closeOtherProductShellMenus", () => {
  it("closes sibling menus but keeps parent drawers open", () => {
    document.body.innerHTML = `
      <header data-product-shell-menu-root>
        <details data-product-shell-menu id="mobile" open>
          <summary>Menu</summary>
          <details data-product-shell-menu id="notifications" open>
            <summary>Notifications</summary>
          </details>
        </details>
        <details data-product-shell-menu id="account" open>
          <summary>Account</summary>
        </details>
      </header>
    `;

    const mobile = document.getElementById("mobile") as HTMLDetailsElement;
    const notifications = document.getElementById("notifications") as HTMLDetailsElement;
    const account = document.getElementById("account") as HTMLDetailsElement;

    closeOtherProductShellMenus(notifications);

    expect(mobile.open).toBe(true);
    expect(notifications.open).toBe(true);
    expect(account.open).toBe(false);
  });
});
