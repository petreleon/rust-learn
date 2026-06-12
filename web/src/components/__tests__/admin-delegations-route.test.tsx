import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { AdminDelegationsRoute } from "@/components/admin-routes";
import { useAdminSession } from "@/components/admin-routes/useAdminSession";
import { createDelegation, fetchDelegations, revokeDelegation, type DelegationItem } from "@/lib/admin";
import { type CurrentSession } from "@/lib/session";

vi.mock("next/navigation", () => ({
  usePathname: () => "/admin/delegations",
  useRouter: () => ({ push: vi.fn() }),
}));

vi.mock("@/components/admin-routes/useAdminSession", () => ({
  useAdminSession: vi.fn(),
}));

vi.mock("@/lib/admin", async (importOriginal) => {
  const actual = await importOriginal<typeof import("@/lib/admin")>();
  return {
    ...actual,
    createDelegation: vi.fn(),
    fetchDelegations: vi.fn(),
    revokeDelegation: vi.fn(),
  };
});

function session(permissions: string[]): CurrentSession {
  return {
    courses: [],
    delegated_permissions: [],
    organizations: [],
    platform: {
      delegated_permissions: [],
      direct_permissions: permissions,
      effective_permissions: permissions,
      roles: permissions.length ? ["platform_admin"] : [],
    },
    user: {
      email: "admin@example.com",
      email_verified: true,
      id: 1,
      kyc_verified: true,
      name: "Admin User",
    },
  };
}

function delegation(overrides: Partial<DelegationItem> = {}): DelegationItem {
  return {
    course_id: null,
    created_at: "2026-06-12T10:00:00Z",
    expires_at: null,
    grantee_user_id: 8,
    grantor_user_id: 1,
    id: 5,
    organization_id: null,
    permission: "APPROVE_REWARD_AMOUNT",
    reason: "Temporary review cover",
    revoked_at: null,
    revoked_by_user_id: null,
    revoke_reason: null,
    scope_type: "platform",
    updated_at: "2026-06-12T10:00:00Z",
    ...overrides,
  };
}

function mockRoute(permissions = ["MANAGE_ROLE_PERMISSIONS"]) {
  vi.mocked(useAdminSession).mockReturnValue({
    error: null,
    hasToken: true,
    loadSession: vi.fn(),
    loadState: "success",
    session: session(permissions),
    signOut: vi.fn(),
    token: "admin-token",
  });
}

describe("AdminDelegationsRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockRoute();
  });

  it("creates a delegation from the empty state", async () => {
    const created = delegation({ id: 10, grantee_user_id: 44, reason: "Coverage" });
    vi.mocked(fetchDelegations).mockResolvedValue({ delegations: [], limit: 100, offset: 0, total: 0 });
    vi.mocked(createDelegation).mockResolvedValue(created);
    const user = userEvent.setup();

    render(<AdminDelegationsRoute />);

    expect(await screen.findByText("No delegated permissions found.")).toBeVisible();
    await user.selectOptions(screen.getByLabelText("Scope type"), "platform");
    await user.type(screen.getByLabelText("Permission"), "APPROVE_REWARD_AMOUNT");
    await user.type(screen.getByLabelText("Grantee user ID"), "44");
    await user.type(screen.getByLabelText("Reason"), "Coverage");
    await user.click(screen.getByRole("button", { name: "Create delegation" }));

    await waitFor(() =>
      expect(createDelegation).toHaveBeenCalledWith({
        course_id: null,
        expires_at: null,
        grantee_user_id: 44,
        organization_id: null,
        permission: "APPROVE_REWARD_AMOUNT",
        reason: "Coverage",
        scope_type: "platform",
        token: "admin-token",
      }),
    );
    expect(await screen.findByText("Delegation created")).toBeVisible();
    expect(screen.getByText("Delegation 10")).toBeVisible();
  });

  it("selects and revokes an active delegation", async () => {
    const row = delegation();
    const revoked = delegation({
      revoked_at: "2026-06-12T11:00:00Z",
      revoked_by_user_id: 1,
      revoke_reason: "Done",
    });
    vi.mocked(fetchDelegations).mockResolvedValue({ delegations: [row], limit: 100, offset: 0, total: 1 });
    vi.mocked(revokeDelegation).mockResolvedValue(revoked);
    const user = userEvent.setup();

    render(<AdminDelegationsRoute />);

    await user.click(await screen.findByRole("button", { name: /Approve Reward Amount/i }));
    expect(screen.getByText("Delegation 5")).toBeVisible();
    await user.type(screen.getByLabelText("Revoke reason"), "Done");
    await user.click(screen.getByRole("button", { name: "Revoke delegation" }));

    await waitFor(() =>
      expect(revokeDelegation).toHaveBeenCalledWith({
        delegationId: 5,
        revokeReason: "Done",
        token: "admin-token",
      }),
    );
    expect(await screen.findByText("Delegation revoked")).toBeVisible();
    expect(screen.getByText("This delegation was revoked and cannot be changed.")).toBeVisible();
  });

  it("shows a permission gate before calling the delegation API", () => {
    mockRoute(["VIEW_REPORT"]);

    render(<AdminDelegationsRoute />);

    expect(screen.getByText("Delegated permissions unavailable")).toBeVisible();
    expect(fetchDelegations).not.toHaveBeenCalled();
  });
});
