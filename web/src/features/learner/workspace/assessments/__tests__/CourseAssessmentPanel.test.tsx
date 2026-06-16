import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import {
  loadCourseAssessmentSnapshot,
  submitCourseAssessmentAttempt,
} from "../api/courseAssessmentApi";
import { CourseAssessmentPanel } from "../route/CourseAssessmentPanel";

vi.mock("@/shared/session/browserSession", () => ({
  clearBrowserSession: vi.fn(),
  readBrowserSessionToken: vi.fn(),
}));

vi.mock("../api/courseAssessmentApi", () => ({
  loadCourseAssessmentSnapshot: vi.fn(),
  submitCourseAssessmentAttempt: vi.fn(),
}));

const assessment = {
  course_id: 9,
  created_at: "2026-06-16T00:00:00Z",
  description: "Confirm the core ownership rule.",
  id: 42,
  max_attempts: 2,
  passing_score: 70,
  published: true,
  questions: [{
    assessment_id: 42,
    id: 101,
    options: ["Ownership", "Prototype chain"],
    order: 0,
    points: 2,
    question_type: "multiple_choice",
    text: "Which Rust concept prevents aliasing bugs?",
  }],
  title: "Ownership check",
  updated_at: "2026-06-16T00:00:00Z",
};

describe("CourseAssessmentPanel", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readBrowserSessionToken).mockReturnValue("learner-token");
    vi.mocked(loadCourseAssessmentSnapshot).mockResolvedValue({
      assessments: [assessment],
      attemptsByAssessmentId: { 42: [] },
    });
  });

  it("loads assessments through the learner feature API", async () => {
    render(<CourseAssessmentPanel courseId={9} submissionsEnabled />);

    expect(await screen.findByRole("heading", { name: "Ownership check" })).toBeVisible();
    expect(screen.getByText("2 attempts left")).toBeVisible();
    await waitFor(() =>
      expect(loadCourseAssessmentSnapshot).toHaveBeenCalledWith({
        courseId: 9,
        token: "learner-token",
      }),
    );
  });

  it("submits a completed answer draft and shows the saved attempt", async () => {
    const user = userEvent.setup();
    vi.mocked(submitCourseAssessmentAttempt).mockResolvedValue({
      attempt: {
        assessment_id: 42,
        completed_at: "2026-06-16T10:00:00Z",
        id: 77,
        passed: true,
        score: 2,
        started_at: "2026-06-16T10:00:00Z",
        user_id: 5,
      },
      passed: true,
      percentage: 100,
      reward_handoff: {
        candidate_id: 91,
        event_type: "assessment_completion",
        message: "Reward review was queued for this assessment completion.",
        policy_id: 12,
        status: "created",
      },
      score: 2,
      total_points: 2,
    });

    render(<CourseAssessmentPanel courseId={9} submissionsEnabled />);

    await user.click(await screen.findByLabelText("Ownership"));
    await user.click(screen.getByRole("button", { name: "Submit attempt" }));

    await waitFor(() =>
      expect(submitCourseAssessmentAttempt).toHaveBeenCalledWith({
        answers: { 101: "Ownership" },
        assessmentId: 42,
        courseId: 9,
        token: "learner-token",
      }),
    );
    expect(await screen.findByText("Assessment passed. Reward review queued.")).toBeVisible();
    expect(screen.getByText("Score 2")).toBeVisible();
  });

  it("keeps assessment attempts disabled in preview mode", async () => {
    render(<CourseAssessmentPanel courseId={9} submissionsEnabled={false} />);

    expect(await screen.findByText("Preview mode shows assessments without allowing attempts.")).toBeVisible();
    expect(await screen.findByLabelText("Ownership")).toBeDisabled();
    expect(await screen.findByRole("button", { name: "Submit attempt" })).toBeDisabled();
  });

  it("disables submission after max attempts", async () => {
    vi.mocked(loadCourseAssessmentSnapshot).mockResolvedValue({
      assessments: [{ ...assessment, max_attempts: 1 }],
      attemptsByAssessmentId: {
        42: [{
          assessment_id: 42,
          completed_at: "2026-06-16T09:00:00Z",
          id: 76,
          passed: false,
          score: 0,
          started_at: "2026-06-16T09:00:00Z",
          user_id: 5,
        }],
      },
    });

    render(<CourseAssessmentPanel courseId={9} submissionsEnabled />);

    expect(await screen.findByText("Max attempts")).toBeVisible();
    expect(await screen.findByLabelText("Ownership")).toBeDisabled();
    expect(await screen.findByRole("button", { name: "Submit attempt" })).toBeDisabled();
  });
});
