import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { AdminFraudBlocksRoute } from "../route/AdminFraudBlocksRoute";
import {
  createAdminFraudBlock,
  loadActiveRewardPolicies,
  loadAdminFraudBlockAudit,
  loadAdminFraudBlocks,
  loadAdminFraudBlockSession,
  revokeAdminFraudBlock,
} from "../api/fraudBlocksApi";
import {
  adminFraudBlockSession,
  fraudBlock,
  fraudBlockAuditEvent,
} from "./adminFraudBlocksTestFixtures";

vi.mock("next/navigation", () => ({
  usePathname: () => "/admin/fraud-blocks",
  useRouter: () => ({ push: vi.fn() }),
}));

vi.mock("@/shared/session/browserSession", () => ({
  clearBrowserSession: vi.fn(),
  readBrowserSessionToken: vi.fn(),
}));

vi.mock("../api/fraudBlocksApi", () => ({
  createAdminFraudBlock: vi.fn(),
  loadActiveRewardPolicies: vi.fn(),
  loadAdminFraudBlockAudit: vi.fn(),
  loadAdminFraudBlocks: vi.fn(),
  loadAdminFraudBlockSession: vi.fn(),
  revokeAdminFraudBlock: vi.fn(),
}));

function mockToken(token: string | null) {
  vi.mocked(readBrowserSessionToken).mockReturnValue(token);
}

describe("AdminFraudBlocksRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockToken("admin-token");
    vi.mocked(loadAdminFraudBlockSession).mockResolvedValue(
      adminFraudBlockSession(["VIEW_REWARD_AUDIT", "MANAGE_REWARD_FRAUD_BLOCKS"]),
    );
    vi.mocked(loadAdminFraudBlocks).mockResolvedValue({
      blocks: [],
      limit: 10,
      offset: 0,
      total: 0,
    });
    vi.mocked(loadAdminFraudBlockAudit).mockResolvedValue([]);
    vi.mocked(loadActiveRewardPolicies).mockResolvedValue([]);
  });

  it("shows the signed-out state without loading fraud block APIs", async () => {
    mockToken(null);

    render(<AdminFraudBlocksRoute />);

    expect(await screen.findByText("Sign in required")).toBeVisible();
    expect(loadAdminFraudBlockSession).not.toHaveBeenCalled();
    expect(loadActiveRewardPolicies).not.toHaveBeenCalled();
    expect(loadAdminFraudBlocks).not.toHaveBeenCalled();
  });

  it("loads fraud blocks and the selected block audit trail", async () => {
    const block = fraudBlock();
    vi.mocked(loadAdminFraudBlocks).mockResolvedValue({
      blocks: [block],
      limit: 10,
      offset: 0,
      total: 1,
    });
    vi.mocked(loadAdminFraudBlockAudit).mockResolvedValue([fraudBlockAuditEvent()]);

    render(<AdminFraudBlocksRoute />);

    expect(await screen.findByText("Block #11")).toBeVisible();
    await waitFor(() =>
      expect(loadAdminFraudBlocks).toHaveBeenCalledWith({
        active: true,
        offset: 0,
        scopeType: "",
        token: "admin-token",
      }),
    );
    await waitFor(() =>
      expect(loadAdminFraudBlockAudit).toHaveBeenCalledWith({ blockId: 11, token: "admin-token" }),
    );
    expect(await screen.findByText("Created by audit rule")).toBeVisible();
  });

  it("creates a fraud block from the empty state", async () => {
    const created = fraudBlock({ id: 12, reason: "Teacher gaming rewards" });
    vi.mocked(loadAdminFraudBlocks)
      .mockResolvedValueOnce({ blocks: [], limit: 10, offset: 0, total: 0 })
      .mockResolvedValueOnce({ blocks: [created], limit: 10, offset: 0, total: 1 });
    vi.mocked(createAdminFraudBlock).mockResolvedValue(created);
    const user = userEvent.setup();

    render(<AdminFraudBlocksRoute />);

    expect(await screen.findByText("No fraud blocks match these filters.")).toBeVisible();
    await user.selectOptions(screen.getByLabelText("Scope type"), "teacher");
    await user.type(screen.getByLabelText("Target ID"), "42");
    await user.type(screen.getByLabelText("Reason"), "Teacher gaming rewards");
    await user.type(screen.getByLabelText("Evidence reference"), "CASE-7");
    await user.click(screen.getByRole("button", { name: "Create block" }));

    await waitFor(() =>
      expect(createAdminFraudBlock).toHaveBeenCalledWith({
        evidence: "CASE-7",
        reason: "Teacher gaming rewards",
        scopeType: "teacher",
        targetId: "42",
        token: "admin-token",
      }),
    );
    expect(await screen.findByText("Block created")).toBeVisible();
    expect(await screen.findByText("Block #12")).toBeVisible();
  });

  it("revokes the selected active fraud block", async () => {
    const activeBlock = fraudBlock();
    const revokedBlock = fraudBlock({
      revoked_at: "2026-06-12T11:00:00Z",
      revoked_by_user_id: 1,
      updated_at: "2026-06-12T11:00:00Z",
    });
    vi.mocked(loadAdminFraudBlocks)
      .mockResolvedValueOnce({ blocks: [activeBlock], limit: 10, offset: 0, total: 1 })
      .mockResolvedValueOnce({ blocks: [revokedBlock], limit: 10, offset: 0, total: 1 });
    vi.mocked(revokeAdminFraudBlock).mockResolvedValue(revokedBlock);
    const user = userEvent.setup();

    render(<AdminFraudBlocksRoute />);

    expect(await screen.findByText("Block 11")).toBeVisible();
    await user.click(screen.getByRole("button", { name: "Revoke" }));

    await waitFor(() =>
      expect(revokeAdminFraudBlock).toHaveBeenCalledWith({ blockId: 11, token: "admin-token" }),
    );
    expect(await screen.findByText("Block revoked")).toBeVisible();
    expect(await screen.findByText("This block was revoked and cannot be changed.")).toBeVisible();
  });

  it("shows a permission gate before calling the fraud block API", async () => {
    vi.mocked(loadAdminFraudBlockSession).mockResolvedValue(adminFraudBlockSession(["VIEW_REPORT"]));

    render(<AdminFraudBlocksRoute />);

    expect(await screen.findByText("Fraud blocks unavailable")).toBeVisible();
    expect(loadActiveRewardPolicies).not.toHaveBeenCalled();
    expect(loadAdminFraudBlocks).not.toHaveBeenCalled();
  });
});
