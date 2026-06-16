import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { type RewardPolicyItem } from "@/lib/admin/RewardPolicyItem";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import {
  createAdminRewardPolicy,
  loadAdminRewardPolicySession,
  loadRewardPolicyItems,
} from "../api/rewardPoliciesApi";
import { AdminRewardPoliciesRoute } from "../route/AdminRewardPoliciesRoute";

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
  loadAdminRewardPolicySession: vi.fn(),
  loadRewardPolicyItems: vi.fn(),
}));

describe("AdminRewardPoliciesRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readBrowserSessionToken).mockReturnValue("admin-token");
    vi.mocked(loadAdminRewardPolicySession).mockResolvedValue(adminRewardPolicySession(["SET_REWARD_POLICY"]));
    vi.mocked(loadRewardPolicyItems).mockResolvedValue([rewardPolicy()]);
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
    await waitFor(() =>
      expect(loadRewardPolicyItems).toHaveBeenCalledWith({
        filters: { active: true, eventType: "", offset: 0, scopeType: "" },
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
  });
});

function adminRewardPolicySession(permissions: string[]): CurrentSession {
  return {
    access: { learner: true, organization: false, platform_admin: permissions.length > 0, teacher: false, teacher_application: false },
    courses: [],
    delegated_permissions: [],
    organizations: [],
    platform: {
      capabilities: [{ enabled: permissions.includes("SET_REWARD_POLICY"), key: "reward_policies", label: "Reward policies", permissions: ["SET_REWARD_POLICY"] }],
      delegated_permissions: [],
      direct_permissions: permissions,
      effective_permissions: permissions,
      roles: permissions.length ? ["platform_admin"] : [],
    },
    user: { email: "admin@example.com", email_verified: true, id: 1, kyc_verified: true, name: "Admin User" },
  };
}

function rewardPolicy(overrides: Partial<RewardPolicyItem> = {}): RewardPolicyItem {
  return {
    active: true,
    cooldown_seconds: 0,
    course_id: null,
    created_at: "2026-06-16T09:00:00Z",
    created_by_user_id: 1,
    event_type: "course_completion",
    id: 41,
    max_payout: null,
    multiplier: "1",
    organization_id: null,
    payment_strategy: "treasury_transfer",
    scope_type: "platform",
    token_amount: "10",
    updated_at: "2026-06-16T09:15:00Z",
    version: 1,
    ...overrides,
  };
}
