import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { AdminDelegationsRoute } from "../route/AdminDelegationsRoute";
import {
  grantAdminDelegation,
  loadAdminDelegations,
  loadAdminDelegationSession,
  revokeAdminDelegation,
} from "../api/delegationsApi";
import { adminDelegationSession, delegation } from "./adminDelegationsTestFixtures";

vi.mock("next/navigation", () => ({
  usePathname: () => "/admin/delegations",
  useRouter: () => ({ push: vi.fn() }),
}));

vi.mock("@/shared/session/browserSession", () => ({
  clearBrowserSession: vi.fn(),
  readBrowserSessionToken: vi.fn(),
}));

vi.mock("../api/delegationsApi", () => ({
  grantAdminDelegation: vi.fn(),
  loadAdminDelegations: vi.fn(),
  loadAdminDelegationSession: vi.fn(),
  revokeAdminDelegation: vi.fn(),
}));

function mockToken(token: string | null) {
  vi.mocked(readBrowserSessionToken).mockReturnValue(token);
}

describe("AdminDelegationsRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockToken("admin-token");
    vi.mocked(loadAdminDelegationSession).mockResolvedValue(
      adminDelegationSession(["VIEW_ROLE_ASSIGNMENTS", "MANAGE_ROLE_PERMISSIONS"]),
    );
    vi.mocked(loadAdminDelegations).mockResolvedValue({
      delegations: [],
      limit: 100,
      offset: 0,
      total: 0,
    });
  });

  it("shows the signed-out state without loading delegation APIs", async () => {
    mockToken(null);

    render(<AdminDelegationsRoute />);

    expect(await screen.findByText("Sign in required")).toBeVisible();
    expect(loadAdminDelegationSession).not.toHaveBeenCalled();
    expect(loadAdminDelegations).not.toHaveBeenCalled();
  });

  it("creates a delegation from the empty state", async () => {
    const created = delegation({ grantee_user_id: 44, id: 10, reason: "Coverage" });
    vi.mocked(grantAdminDelegation).mockResolvedValue(created);
    const user = userEvent.setup();

    render(<AdminDelegationsRoute />);

    expect(await screen.findByText("No delegated permissions found.")).toBeVisible();
    await user.selectOptions(screen.getByLabelText("Scope type"), "platform");
    await user.type(screen.getByLabelText("Permission"), "APPROVE_REWARD_AMOUNT");
    await user.type(screen.getByLabelText("Grantee user ID"), "44");
    await user.type(screen.getByLabelText("Reason"), "Coverage");
    await user.click(screen.getByRole("button", { name: "Create delegation" }));

    await waitFor(() =>
      expect(grantAdminDelegation).toHaveBeenCalledWith({
        courseId: "",
        expiresAt: "",
        granteeUserId: "44",
        organizationId: "",
        permission: "APPROVE_REWARD_AMOUNT",
        reason: "Coverage",
        scopeType: "platform",
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
    vi.mocked(loadAdminDelegations).mockResolvedValue({ delegations: [row], limit: 100, offset: 0, total: 1 });
    vi.mocked(revokeAdminDelegation).mockResolvedValue(revoked);
    const user = userEvent.setup();

    render(<AdminDelegationsRoute />);

    await user.click(await screen.findByRole("button", { name: /Approve Reward Amount/i }));
    expect(screen.getByText("Delegation 5")).toBeVisible();
    await user.type(screen.getByLabelText("Revoke reason"), "Done");
    await user.click(screen.getByRole("button", { name: "Revoke delegation" }));

    await waitFor(() =>
      expect(revokeAdminDelegation).toHaveBeenCalledWith({
        delegationId: 5,
        revokeReason: "Done",
        token: "admin-token",
      }),
    );
    expect(await screen.findByText("Delegation revoked")).toBeVisible();
    expect(screen.getByText("This delegation was revoked and cannot be changed.")).toBeVisible();
  });

  it("shows a permission gate before calling the delegation API", async () => {
    vi.mocked(loadAdminDelegationSession).mockResolvedValue(adminDelegationSession(["VIEW_REPORT"]));

    render(<AdminDelegationsRoute />);

    expect(await screen.findByText("Delegated permissions unavailable")).toBeVisible();
    expect(loadAdminDelegations).not.toHaveBeenCalled();
  });
});
