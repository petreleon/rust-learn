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
  rewardPolicy,
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

describe("AdminFraudBlocksRoute reward policy picker", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readBrowserSessionToken).mockReturnValue("admin-token");
    vi.mocked(loadAdminFraudBlockSession).mockResolvedValue(
      adminFraudBlockSession(["VIEW_REWARD_AUDIT", "MANAGE_REWARD_FRAUD_BLOCKS", "SET_REWARD_POLICY"]),
    );
    vi.mocked(loadAdminFraudBlocks).mockResolvedValue({ blocks: [], limit: 10, offset: 0, total: 0 });
    vi.mocked(loadAdminFraudBlockAudit).mockResolvedValue([]);
    vi.mocked(loadActiveRewardPolicies).mockResolvedValue([rewardPolicy()]);
  });

  it("creates reward-policy scoped fraud blocks from active policy options", async () => {
    const created = fraudBlock({
      id: 13,
      reward_policy_id: 401,
      scope_type: "reward_policy",
      teacher_user_id: null,
    });
    vi.mocked(loadAdminFraudBlocks)
      .mockResolvedValueOnce({ blocks: [], limit: 10, offset: 0, total: 0 })
      .mockResolvedValueOnce({ blocks: [created], limit: 10, offset: 0, total: 1 });
    vi.mocked(createAdminFraudBlock).mockResolvedValue(created);
    const user = userEvent.setup();

    render(<AdminFraudBlocksRoute />);

    expect(await screen.findByText("No fraud blocks match these filters.")).toBeVisible();
    await waitFor(() => expect(loadActiveRewardPolicies).toHaveBeenCalledWith({ token: "admin-token" }));
    await user.selectOptions(screen.getByLabelText("Scope type"), "reward_policy");
    await user.selectOptions(screen.getByLabelText("Reward policy"), "401");
    await user.type(screen.getByLabelText("Reason"), "Policy payout abuse");
    await user.click(screen.getByRole("button", { name: "Create block" }));

    await waitFor(() =>
      expect(createAdminFraudBlock).toHaveBeenCalledWith({
        evidence: "",
        reason: "Policy payout abuse",
        scopeType: "reward_policy",
        targetId: "401",
        token: "admin-token",
      }),
    );
    expect(await screen.findByText("Reward policy 401")).toBeVisible();
    expect(revokeAdminFraudBlock).not.toHaveBeenCalled();
  });
});
