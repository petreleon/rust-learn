import { type PlatformRewardCandidateItem } from "@/lib/admin/PlatformRewardCandidateItem";
import { type PlatformRewardCandidatesResponse } from "@/lib/admin/PlatformRewardCandidatesResponse";
import { type RewardAuditEvent } from "@/lib/admin/RewardAuditEvent";
import { type CurrentSession } from "@/lib/session/CurrentSession";

const rewardAmountPermissions = ["VIEW_REWARD_AUDIT", "APPROVE_REWARD_AMOUNT"];

export function adminRewardAmountSession(permissions: string[]): CurrentSession {
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
          enabled: rewardAmountPermissions.some((permission) => permissions.includes(permission)),
          key: "reward_amount_review",
          label: "Reward amount review",
          permissions: rewardAmountPermissions,
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

export function rewardCandidate(
  overrides: Partial<PlatformRewardCandidateItem> = {},
): PlatformRewardCandidateItem {
  return {
    approved_amount: null,
    course: { id: 4, title: "Rust Foundations" },
    created_at: "2026-06-12T10:00:00Z",
    event_type: "lesson_completed",
    id: 31,
    source_organization_id: 2,
    source_scope: "organization",
    status: "teacher_approved",
    student: { email: "ada@example.com", id: 22, name: "Ada Student" },
    submitter: { email: "teacher@example.com", id: 7, name: "Teacher User" },
    teacher_approver: { email: "reviewer@example.com", id: 8, name: "Reviewer User" },
    teacher_decision_reason: "Teacher approved evidence",
    updated_at: "2026-06-12T10:00:00Z",
    ...overrides,
  };
}

export function rewardCandidateResponse(
  candidates: PlatformRewardCandidateItem[],
): PlatformRewardCandidatesResponse {
  return {
    candidates,
    limit: 8,
    offset: 0,
    operator_permissions: {
      can_approve_amount: true,
      can_view_candidates: true,
    },
    search: null,
    status: "teacher_approved",
    total: candidates.length,
  };
}

export function rewardAuditEvent(overrides: Partial<RewardAuditEvent> = {}): RewardAuditEvent {
  return {
    actor_user_id: 8,
    created_at: "2026-06-12T10:05:00Z",
    event_type: "teacher_decision",
    from_status: "pending_teacher_approval",
    id: 14,
    metadata: {},
    reason: "Reviewed by teacher",
    reward_candidate_id: 31,
    to_status: "teacher_approved",
    ...overrides,
  };
}
