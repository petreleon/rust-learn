import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import {
  createAdminRewardPolicy,
  loadAdminRewardPolicyAudit,
  loadAdminRewardPolicySession,
  loadRewardPolicyItems,
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

describe("AdminRewardPoliciesRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readBrowserSessionToken).mockReturnValue("admin-token");
    vi.mocked(loadAdminRewardPolicySession).mockResolvedValue(adminRewardPolicySession(["SET_REWARD_POLICY"]));
    vi.mocked(loadRewardPolicyItems).mockResolvedValue([rewardPolicy()]);
    vi.mocked(loadAdminRewardPolicyAudit).mockResolvedValue([rewardPolicyAuditEvent()]);
    vi.mocked(createAdminRewardPolicy).mockResolvedValue(rewardPolicy({ id: 99, scope_type: "course" }));
  });

  it("shows signed-out state without loading reward policy APIs", async () => {
    vi.mocked(readBrowserSessionToken).mockReturnValue(null);

    render(<AdminRewardPoliciesRoute />);

    expect(await screen.findByText("Sign in required")).toBeVisible();
    expect(loadAdminRewardPolicySession).not.toHaveBeenCalled();
    expect(loadRewardPolicyItems).not.toHaveBeenCalled();
  });

  it("loads reward policies through the feature API boundary", async () => {
    render(<AdminRewardPoliciesRoute />);

    expect(await screen.findByText("Policy versions")).toBeVisible();
    expect(await screen.findByText("Platform policy #41")).toBeVisible();
    expect(await screen.findByText("Policy inspection")).toBeVisible();
    expect(screen.getByText("#41")).toBeVisible();
    expect(await screen.findByText("Policy audit")).toBeVisible();
    expect((await screen.findAllByText("Created")).length).toBeGreaterThan(0);
    await waitFor(() =>
      expect(loadRewardPolicyItems).toHaveBeenCalledWith({
        filters: { active: true, eventType: "", offset: 0, scopeType: "" },
        token: "admin-token",
      }),
    );
    await waitFor(() =>
      expect(loadAdminRewardPolicyAudit).toHaveBeenCalledWith({
        policyId: 41,
        token: "admin-token",
      }),
    );
  });

  it("inspects a selected policy version from the loaded list", async () => {
    const user = userEvent.setup();
    vi.mocked(loadRewardPolicyItems).mockResolvedValue([
      rewardPolicy(),
      rewardPolicy({
        active: false,
        cooldown_seconds: 60,
        course_id: 9,
        event_type: "assessment_completion",
        id: 42,
        max_payout: "100",
        payment_strategy: "off_chain",
        scope_type: "course",
        version: 2,
      }),
    ]);

    render(<AdminRewardPoliciesRoute />);

    expect(await screen.findByText("#41")).toBeVisible();
    await user.click(screen.getByRole("button", { name: "Inspect policy #42" }));

    expect(await screen.findByText("#42")).toBeVisible();
    expect(screen.getAllByText("Course 9").length).toBeGreaterThan(0);
    expect(screen.getByText("Assessment Completion")).toBeVisible();
    expect(screen.getByText("100 tokens")).toBeVisible();
    expect(screen.getByText("Historical version")).toBeVisible();
    await waitFor(() =>
      expect(loadAdminRewardPolicyAudit).toHaveBeenCalledWith({
        policyId: 42,
        token: "admin-token",
      }),
    );
  });

  it("filters policies and creates a course-scoped policy", async () => {
    const user = userEvent.setup();
    vi.mocked(loadRewardPolicyItems)
      .mockResolvedValueOnce([rewardPolicy()])
      .mockResolvedValueOnce([rewardPolicy({ scope_type: "course", course_id: 9 })])
      .mockResolvedValueOnce([rewardPolicy({ id: 99, scope_type: "course", course_id: 9 })]);

    render(<AdminRewardPoliciesRoute />);

    await user.selectOptions(await screen.findByLabelText("Scope"), "course");
    await user.selectOptions(screen.getByLabelText("Event"), "assessment_completion");
    await user.selectOptions(screen.getByLabelText("Status"), "");
    await user.click(screen.getByRole("button", { name: "Apply" }));
    await waitFor(() =>
      expect(loadRewardPolicyItems).toHaveBeenCalledWith({
        filters: { active: null, eventType: "assessment_completion", offset: 0, scopeType: "course" },
        token: "admin-token",
      }),
    );

    await user.selectOptions(screen.getByLabelText("Scope type"), "course");
    await user.type(screen.getByLabelText("Organization ID"), "3");
    await user.type(screen.getByLabelText("Course ID"), "9");
    await user.selectOptions(screen.getByLabelText("Event type"), "assessment_completion");
    await user.selectOptions(screen.getByLabelText("Payment strategy"), "off_chain");
    await user.type(screen.getByLabelText("Token amount"), "25");
    await user.click(screen.getByRole("button", { name: "Create policy" }));

    await waitFor(() =>
      expect(createAdminRewardPolicy).toHaveBeenCalledWith({
        payload: {
          active: true,
          cooldown_seconds: 0,
          course_id: 9,
          event_type: "assessment_completion",
          max_payout: null,
          multiplier: "1",
          organization_id: 3,
          payment_strategy: "off_chain",
          scope_type: "course",
          token_amount: "25",
        },
        token: "admin-token",
      }),
    );
    expect(await screen.findByText("Policy created")).toBeVisible();
  });

  it("shows a permission gate before loading policies", async () => {
    vi.mocked(loadAdminRewardPolicySession).mockResolvedValue(adminRewardPolicySession(["VIEW_REPORT"]));

    render(<AdminRewardPoliciesRoute />);

    expect(await screen.findByText("Reward policies unavailable")).toBeVisible();
    expect(loadRewardPolicyItems).not.toHaveBeenCalled();
    expect(loadAdminRewardPolicyAudit).not.toHaveBeenCalled();
  });
});
