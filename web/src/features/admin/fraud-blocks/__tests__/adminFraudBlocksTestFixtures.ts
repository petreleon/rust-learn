import { type FraudBlockAuditEvent } from "@/lib/admin/FraudBlockAuditEvent";
import { type FraudBlockItem } from "@/lib/admin/FraudBlockItem";
import { type RewardPolicyItem } from "@/lib/admin/RewardPolicyItem";
import { type CurrentSession } from "@/lib/session/CurrentSession";

const fraudBlockPermissions = [
  "VIEW_REWARD_AUDIT",
  "MANAGE_REWARD_FRAUD_BLOCKS",
  "BLOCK_REWARD_TEACHER",
  "BLOCK_REWARD_ORGANIZATION",
];

export function adminFraudBlockSession(permissions: string[]): CurrentSession {
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
      capabilities: [
        {
          enabled: fraudBlockPermissions.some((permission) => permissions.includes(permission)),
          key: "fraud_blocks",
          label: "Fraud blocks",
          permissions: fraudBlockPermissions,
        },
      ],
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

export function fraudBlock(overrides: Partial<FraudBlockItem> = {}): FraudBlockItem {
  return {
    course_id: null,
    created_at: "2026-06-12T10:00:00Z",
    created_by_user_id: 1,
    evidence_reference: "CASE-1",
    expires_at: null,
    id: 11,
    organization_id: null,
    reason: "Suspicious reward pattern",
    reward_policy_id: null,
    revoked_at: null,
    revoked_by_user_id: null,
    scope_type: "teacher",
    teacher_user_id: 42,
    updated_at: "2026-06-12T10:00:00Z",
    ...overrides,
  };
}

export function fraudBlockAuditEvent(
  overrides: Partial<FraudBlockAuditEvent> = {},
): FraudBlockAuditEvent {
  return {
    actor_user_id: 1,
    created_at: "2026-06-12T10:05:00Z",
    event_type: "created",
    fraud_block_id: 11,
    id: 8,
    reason: "Created by audit rule",
    ...overrides,
  };
}

export function rewardPolicy(overrides: Partial<RewardPolicyItem> = {}): RewardPolicyItem {
  return {
    active: true,
    cooldown_seconds: 3600,
    course_id: 9,
    created_at: "2026-06-12T09:00:00Z",
    created_by_user_id: 1,
    event_type: "assessment_passed",
    id: 401,
    max_payout: null,
    multiplier: "1",
    organization_id: null,
    payment_strategy: "off_chain",
    scope_type: "course",
    token_amount: "25",
    updated_at: "2026-06-12T09:00:00Z",
    version: 2,
    ...overrides,
  };
}
