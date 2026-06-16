import { render, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { createTeacherCourseAssessment } from "../api/assessmentAuthoringApi";
import { loadTeacherCourseContentWorkspace } from "../api/courseContentApi";
import { TeacherCourseContentRoute } from "../route/TeacherCourseContentRoute";
import {
  courseContentAssessment,
  courseContentSession,
  courseContentWorkspace,
} from "./courseContentTestFixtures";

vi.mock("next/navigation", () => ({
  usePathname: () => "/teach/courses/9/content",
  useRouter: () => ({ push: vi.fn() }),
}));

vi.mock("@/shared/session/browserSession", () => ({
  clearBrowserSession: vi.fn(),
  readBrowserSessionToken: vi.fn(),
}));

vi.mock("../api/courseContentApi", () => ({
  createTeacherCourseContentChapter: vi.fn(),
  createTeacherCourseContentItem: vi.fn(),
  deleteTeacherCourseContentItem: vi.fn(),
  loadTeacherCourseContentWorkspace: vi.fn(),
  processTeacherCourseContentItem: vi.fn(),
  requestTeacherContentUploadUrl: vi.fn(),
  updateTeacherCourseContentItem: vi.fn(),
  uploadTeacherContentFile: vi.fn(),
}));

vi.mock("../api/assessmentAuthoringApi", () => ({
  createTeacherCourseAssessment: vi.fn(),
  updateTeacherCourseAssessment: vi.fn(),
}));

describe("TeacherCourseAssessmentAuthoringRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readBrowserSessionToken).mockReturnValue("teacher-token");
    vi.mocked(loadTeacherCourseContentWorkspace).mockResolvedValue({
      assessments: [courseContentAssessment()],
      session: courseContentSession(),
      workspace: courseContentWorkspace(),
    });
    vi.mocked(createTeacherCourseAssessment).mockResolvedValue(courseContentAssessment());
  });

  it("creates a published assessment through the feature API boundary", async () => {
    const user = userEvent.setup();
    vi.mocked(loadTeacherCourseContentWorkspace)
      .mockResolvedValueOnce({
        assessments: [courseContentAssessment()],
        session: courseContentSession(),
        workspace: courseContentWorkspace(),
      })
      .mockResolvedValueOnce({
        assessments: [courseContentAssessment({ published: true, title: "Published quiz" })],
        session: courseContentSession(),
        workspace: courseContentWorkspace(),
      });
    render(<TeacherCourseContentRoute courseId="9" />);
    const assessmentPanel = (await screen.findByRole("heading", { name: "Assessment authoring" })).closest("section") as HTMLElement;

    expect(within(assessmentPanel).getByText(/Draft - 70% pass/)).toBeVisible();
    expect(within(assessmentPanel).getByText("Answer key: Ownership")).toBeVisible();
    await user.type(within(assessmentPanel).getByLabelText("Title"), "Ownership quiz");
    await user.type(within(assessmentPanel).getByLabelText("Question 1"), "Which concept prevents aliasing bugs?");
    await user.type(within(assessmentPanel).getByLabelText("Options 1"), "Ownership, Prototype chains");
    await user.type(within(assessmentPanel).getByLabelText("Correct answer 1"), "Ownership");
    await user.click(within(assessmentPanel).getByRole("button", { name: "Add question" }));
    await user.type(within(assessmentPanel).getByLabelText("Question 2"), "What does borrow checking protect?");
    await user.type(within(assessmentPanel).getByLabelText("Correct answer 2"), "Aliasing");
    await user.click(within(assessmentPanel).getByLabelText("Published for learners"));
    await user.click(within(assessmentPanel).getByRole("button", { name: "Create assessment" }));

    await waitFor(() =>
      expect(createTeacherCourseAssessment).toHaveBeenCalledWith({
        courseId: "9",
        payload: {
          description: null,
          max_attempts: 3,
          passing_score: 70,
          published: true,
          questions: [{
            correct_answer: "Ownership",
            options: ["Ownership", "Prototype chains"],
            order: 0,
            points: 1,
            question_type: "multiple_choice",
            text: "Which concept prevents aliasing bugs?",
          }, {
            correct_answer: "Aliasing",
            options: null,
            order: 1,
            points: 1,
            question_type: "multiple_choice",
            text: "What does borrow checking protect?",
          }],
          title: "Ownership quiz",
        },
        token: "teacher-token",
      }),
    );
    expect(await screen.findByText("Assessment created.")).toBeVisible();
    expect(await screen.findByText(/Published - 70% pass/)).toBeVisible();
  });
});
