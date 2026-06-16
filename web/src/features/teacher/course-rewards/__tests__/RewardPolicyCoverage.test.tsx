import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { type TeacherCourseDashboardItem } from "@/lib/teacher/TeacherCourseDashboardItem";
import { type TeacherRewardCandidate } from "@/lib/teacher/TeacherRewardCandidate";
import { RewardCandidateCard } from "../components/RewardCandidateCard";
import { RewardPolicyCoveragePanel } from "../components/RewardPolicyCoveragePanel";
import { defaultRewardDecisionDraft } from "../model/defaultRewardDecisionDraft";

const courseBase: TeacherCourseDashboardItem = {
  content: { chapter_count: 1, content_count: 2, content_types: ["article"], has_content: true },
  id: 9,
  lifecycle_status: "published",
  organizations: [],
  permissions: {
    can_approve_reward_candidates: true,
    can_manage_content: true,
    can_manage_enrollments: true,
    can_manage_reward_rules: false,
    can_manage_settings: true,
    can_view_reward_candidates: true,
  },
  reward_queue: { failed_count: 0, pending_teacher_count: 1, teacher_approved_count: 0 },
  rewards: {
    active_policy_count: 0,
    available: false,
    event_types: [],
    payment_strategies: [],
    token_amounts: [],
  },
  roster: { enrolled_student_count: 1, pending_join_request_count: 0, waitlisted_join_request_count: 0 },
  title: "Rust Safety",
};

const candidate: TeacherRewardCandidate = {
  amount_decided_at: null,
  amount_decision_reason: null,
  amount_reviewer_user_id: null,
  approved_amount: null,
  course_id: 9,
  created_at: "2026-06-12T10:00:00Z",
  event_type: "assessment_passed",
  evidence: { assessment_id: 5 },
  id: 77,
  idempotency_key: "candidate-77",
  source_organization_id: null,
  source_scope: "course",
  status: "pending_teacher_approval",
  student_user_id: 42,
  submitter_user_id: 2,
  teacher_approver_user_id: null,
  teacher_decided_at: null,
  teacher_decision_reason: null,
  updated_at: "2026-06-12T10:00:00Z",
};

describe("reward policy coverage", () => {
  it("explains missing policy coverage on the reward review route", () => {
    render(<RewardPolicyCoveragePanel course={courseBase} />);

    expect(screen.getByText(/No active reward policy covers this course yet/)).toBeVisible();
  });

  it("shows the policy coverage that makes a candidate eligible", () => {
    const course = {
      ...courseBase,
      rewards: {
        active_policy_count: 1,
        available: true,
        event_types: ["assessment_passed"],
        payment_strategies: ["off_chain"],
        token_amounts: ["25"],
      },
    };

    render(
      <RewardCandidateCard
        actionState="idle"
        canApprove
        candidate={candidate}
        course={course}
        draft={defaultRewardDecisionDraft}
        learner={{ email: "learner@example.com", id: 42, name: "Learner" }}
        onDraftChange={vi.fn()}
        onSubmit={vi.fn()}
      />,
    );

    expect(
      screen.getByText("Eligible through an active Assessment Passed policy. Active course policy amounts: 25 tokens."),
    ).toBeVisible();
  });

  it("explains when active policies do not cover the candidate event", () => {
    const course = {
      ...courseBase,
      rewards: {
        active_policy_count: 1,
        available: true,
        event_types: ["lesson_completed"],
        payment_strategies: ["off_chain"],
        token_amounts: ["10"],
      },
    };

    render(
      <RewardCandidateCard
        actionState="idle"
        canApprove
        candidate={candidate}
        course={course}
        draft={defaultRewardDecisionDraft}
        learner={{ email: "learner@example.com", id: 42, name: "Learner" }}
        onDraftChange={vi.fn()}
        onSubmit={vi.fn()}
      />,
    );

    expect(screen.getByText("Active policies exist, but none currently cover Assessment Passed.")).toBeVisible();
  });
});
