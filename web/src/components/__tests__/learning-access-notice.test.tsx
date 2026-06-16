import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { LearningAccessNotice } from "@/features/learner/workspace/components/LearningAccessNotice";
import { type CourseLearningResponse } from "@/lib/learner";

function makeLearning(state: string): CourseLearningResponse {
  return {
    active_content_id: null,
    chapters: [],
    course: {
      access: { can_request_join: false, can_view_content: true, can_view_course: true, can_view_rewards: false },
      content: { chapter_count: 0, content_count: 0, content_types: [], has_content: false },
      description: null,
      enrollment: { can_request_join: false, reason: "Enrollment is managed by course staff.", request_id: null, roles: [], state },
      id: 7,
      lifecycle_status: "published",
      organizations: [],
      prerequisites: [],
      rewards: { active_policy_count: 0, available: false, event_types: [], payment_strategies: [], token_amounts: [] },
      teachers: [],
      title: "Ownership Basics",
      topics: [],
    },
    progress_supported: state === "enrolled",
  };
}

describe("LearningAccessNotice", () => {
  it("stays hidden for enrolled learners", () => {
    render(<LearningAccessNotice learning={makeLearning("enrolled")} />);
    expect(screen.queryByText("Preview mode")).not.toBeInTheDocument();
  });

  it("explains read-only preview for non-enrolled content access", () => {
    render(<LearningAccessNotice learning={makeLearning("available")} />);
    expect(screen.getByText("Preview mode")).toBeVisible();
    expect(screen.getByText("available")).toBeVisible();
    expect(screen.getByText(/progress is saved only after course enrollment/i)).toBeVisible();
  });

  it("keeps pending join requests distinct from normal previews", () => {
    render(<LearningAccessNotice learning={makeLearning("pending")} />);
    expect(screen.getByText("pending")).toBeVisible();
    expect(screen.getByText(/join request is still waiting for review/i)).toBeVisible();
  });
});
