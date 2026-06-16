import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import {
  createAdminRewardPolicy,
  loadAdminRewardPolicyAudit,
  loadAdminRewardPolicySession,
  loadRewardPolicyItems,
  updateAdminRewardPolicyActivation,
} from "../api/rewardPoliciesApi";
import { AdminRewardPoliciesRoute } from "../route/AdminRewardPoliciesRoute";
import {
  adminRewardPolicySession,
  rewardPolicy,
  rewardPolicyAuditEvent,
} from "./adminRewardPoliciesTestFixtures";

vi.mock("next/navigation", () => ({
  usePathname: () => "/admin/reward-policies",
  useRouter: () => ({ push: vi.fn() }),
}));

vi.mock("@/shared/session/browserSession", () => ({
  clearBrowserSession: vi.fn(),
  readBrowserSessionToken: vi.fn(),
}));

vi.mock("../api/rewardPoliciesApi", () => ({
  createAdminRewardPolicy: vi.fn(),
  loadAdminRewardPolicyAudit: vi.fn(),
  loadAdminRewardPolicySession: vi.fn(),
  loadRewardPolicyItems: vi.fn(),
  updateAdminRewardPolicyActivation: vi.fn(),
}));

describe("AdminRewardPolicyActivationRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readBrowserSessionToken).mockReturnValue("admin-token");
    vi.mocked(loadAdminRewardPolicySession).mockResolvedValue(adminRewardPolicySession(["SET_REWARD_POLICY"]));
    vi.mocked(loadRewardPolicyItems).mockResolvedValue([rewardPolicy()]);
    vi.mocked(loadAdminRewardPolicyAudit).mockResolvedValue([rewardPolicyAuditEvent()]);
    vi.mocked(createAdminRewardPolicy).mockResolvedValue(rewardPolicy());
    vi.mocked(updateAdminRewardPolicyActivation).mockResolvedValue(rewardPolicy({ active: false }));
  });

  it("deactivates the selected policy through the feature API boundary", async () => {
    const user = userEvent.setup();

    render(<AdminRewardPoliciesRoute />);

    await screen.findByText("Platform policy #41");
    await user.click(screen.getByRole("button", { name: "Deactivate" }));

    await waitFor(() =>
      expect(updateAdminRewardPolicyActivation).toHaveBeenCalledWith({
        active: false,
        policyId: 41,
        token: "admin-token",
      }),
    );
    expect(await screen.findByText("Policy updated")).toBeVisible();
    await waitFor(() => expect(loadRewardPolicyItems).toHaveBeenCalledTimes(2));
  });
});
