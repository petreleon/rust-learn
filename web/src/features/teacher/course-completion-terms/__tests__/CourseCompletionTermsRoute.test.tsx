import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import {
  loadCourseCompletionTerms,
  submitCourseCompletionTerms,
} from "../api/completionTermsApi";
import { CourseCompletionTermsRoute } from "../route/CourseCompletionTermsRoute";
import {
  courseWorkspace,
  courseWorkspaceSession,
} from "../../course-workspace/__tests__/courseWorkspaceTestFixtures";

vi.mock("@/shared/session/browserSession", () => ({
  readBrowserSessionToken: vi.fn(),
}));

vi.mock("../api/completionTermsApi", () => ({
  acceptCourseCompletionTerms: vi.fn(),
  counterCourseCompletionTerms: vi.fn(),
  loadCourseCompletionTerms: vi.fn(),
  rejectCourseCompletionTerms: vi.fn(),
  submitCourseCompletionTerms: vi.fn(),
  withdrawCourseCompletionTerms: vi.fn(),
}));

describe("CourseCompletionTermsRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readBrowserSessionToken).mockReturnValue("teacher-token");
    vi.mocked(loadCourseCompletionTerms).mockResolvedValue({
      active_terms: null,
      audit_events: [],
      terms: [],
    });
    vi.mocked(submitCourseCompletionTerms).mockResolvedValue({
      accepted_at: null,
      accepted_by_user_id: null,
      activated_at: null,
      completion_reward_amount: "10",
      course_id: 9,
      created_at: "2026-06-16T00:00:00Z",
      id: 1,
      max_enrolled_students: 25,
      organization_id: 3,
      proposed_by_user_id: 2,
      reward_policy_id: null,
      status: "submitted",
      superseded_at: null,
      teacher_user_id: 2,
      updated_at: "2026-06-16T00:00:00Z",
      version: 1,
    });
  });

  it("submits a course-specific completion terms proposal", async () => {
    const user = userEvent.setup();
    render(
      <CourseCompletionTermsRoute
        session={courseWorkspaceSession()}
        workspace={courseWorkspace()}
      />,
    );

    await user.type(await screen.findByLabelText("Reward per completion"), "10");
    await user.type(screen.getByLabelText("Max students"), "25");
    await user.type(screen.getByLabelText("Negotiation note"), "First cohort");
    await user.click(screen.getByRole("button", { name: "Submit proposal" }));

    await waitFor(() =>
      expect(submitCourseCompletionTerms).toHaveBeenCalledWith({
        courseId: 9,
        payload: {
          completion_reward_amount: "10",
          max_enrolled_students: 25,
          note: "First cohort",
        },
        token: "teacher-token",
      }),
    );
  });
});
