import { cleanup, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { TeacherRequestError } from "@/lib/teacher";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import {
  createTeacherCourseContentItem,
  loadTeacherCourseContentWorkspace,
} from "../api/courseContentApi";
import { TeacherCourseContentRoute } from "../route/TeacherCourseContentRoute";
import { courseContentSession, courseContentWorkspace } from "./courseContentTestFixtures";

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
  loadTeacherContentProcessingHistory: vi.fn(),
  loadTeacherCourseContentWorkspace: vi.fn(),
  processTeacherCourseContentItem: vi.fn(),
  requestTeacherContentUploadUrl: vi.fn(),
  setTeacherCourseContentPublicationStatus: vi.fn(),
  updateTeacherCourseContentItem: vi.fn(),
  uploadTeacherContentFile: vi.fn(),
}));

describe("TeacherCourseContentRoute create states", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readBrowserSessionToken).mockReturnValue("teacher-token");
    vi.mocked(loadTeacherCourseContentWorkspace).mockResolvedValue({
      assessments: [],
      session: courseContentSession(),
      workspace: courseContentWorkspace(),
    });
    vi.mocked(createTeacherCourseContentItem).mockResolvedValue({
      chapter_id: 3,
      content_type: "article",
      data: "New lesson body",
      id: 21,
      order: 1,
      publication_status: "published",
    });
  });

  it("validates text content before calling the feature API", async () => {
    const user = userEvent.setup();
    render(<TeacherCourseContentRoute courseId="9" />);

    await user.click(await screen.findByRole("button", { name: "Create content" }));

    expect(await screen.findByText("Lesson body is required for text content.")).toBeVisible();
    expect(createTeacherCourseContentItem).not.toHaveBeenCalled();
  });

  it("disables creation controls while the create request is saving", async () => {
    const user = userEvent.setup();
    vi.mocked(createTeacherCourseContentItem).mockReturnValue(new Promise(() => {}));
    render(<TeacherCourseContentRoute courseId="9" />);

    await user.type(await screen.findByLabelText("Lesson body"), "New lesson body");
    await user.click(screen.getByRole("button", { name: "Create content" }));

    expect(await screen.findByRole("button", { name: "Create content" })).toBeDisabled();
  });

  it("surfaces denied, conflict, and backend create failures", async () => {
    const user = userEvent.setup();
    const failures = [
      new TeacherRequestError("You cannot manage content for this course.", 403, "permission_denied"),
      new TeacherRequestError("Content order changed. Refresh and retry.", 409, "conflict"),
      new TeacherRequestError("Content service failed while saving.", 500, "server_error"),
    ];

    for (const failure of failures) {
      vi.mocked(createTeacherCourseContentItem).mockRejectedValueOnce(failure);
      render(<TeacherCourseContentRoute courseId="9" />);

      await user.type(await screen.findByLabelText("Lesson body"), "New lesson body");
      await user.click(screen.getByRole("button", { name: "Create content" }));

      expect(await screen.findByText(failure.message)).toBeVisible();
      cleanup();
    }

    await waitFor(() => expect(createTeacherCourseContentItem).toHaveBeenCalledTimes(3));
  });
});
