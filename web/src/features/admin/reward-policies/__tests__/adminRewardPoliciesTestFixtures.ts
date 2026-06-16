import { type RewardPolicyItem } from "@/lib/admin/RewardPolicyItem";
import { type CurrentSession } from "@/lib/session/CurrentSession";

export function adminRewardPolicySession(permissions: string[]): CurrentSession {
  return {
    access: {
      learner: true,
      organization: false,
      platform_admin: permissions.length > 0,
      teacher: false,
      teacher_application: false,
    },
    courses: [],
    delegated_permissions: [],
    organizations: [],
    platform: {
      capabilities: [{
        enabled: permissions.includes("SET_REWARD_POLICY"),
        key: "reward_policies",
        label: "Reward policies",
        permissions: ["SET_REWARD_POLICY"],
      }],
      delegated_permissions: [],
      direct_permissions: permissions,
      effective_permissions: permissions,
      roles: permissions.length ? ["platform_admin"] : [],
    },
    user: { email: "admin@example.com", email_verified: true, id: 1, kyc_verified: true, name: "Admin User" },
  };
}

export function rewardPolicy(overrides: Partial<RewardPolicyItem> = {}): RewardPolicyItem {
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
