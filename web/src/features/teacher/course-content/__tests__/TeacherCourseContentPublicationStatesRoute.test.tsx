import { cleanup, render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { TeacherRequestError } from "@/lib/teacher";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import {
  loadTeacherCourseContentWorkspace,
  setTeacherCourseContentPublicationStatus,
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

describe("TeacherCourseContentRoute publication states", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readBrowserSessionToken).mockReturnValue("teacher-token");
    vi.mocked(loadTeacherCourseContentWorkspace).mockResolvedValue({
      assessments: [],
      session: courseContentSession(),
      workspace: courseContentWorkspace(),
    });
    vi.mocked(setTeacherCourseContentPublicationStatus).mockResolvedValue({
      chapter_id: 3,
      content_type: "article",
      data: "Original lesson",
      id: 11,
      order: 0,
      publication_status: "unpublished",
    });
  });

  it("disables publication controls while the unpublish request is saving", async () => {
    const user = userEvent.setup();
    vi.mocked(setTeacherCourseContentPublicationStatus).mockReturnValue(new Promise(() => {}));
    render(<TeacherCourseContentRoute courseId="9" />);

    await user.click(await screen.findByRole("button", { name: "Unpublish" }));

    expect(await screen.findByRole("button", { name: "Unpublish" })).toBeDisabled();
  });

  it("surfaces denied, conflict, and backend unpublish failures", async () => {
    const user = userEvent.setup();
    const failures = [
      new TeacherRequestError("You cannot publish content for this course.", 403, "permission_denied"),
      new TeacherRequestError("Content was updated by another author.", 409, "conflict"),
      new TeacherRequestError("Content publication failed.", 500, "server_error"),
    ];

    for (const failure of failures) {
      vi.mocked(setTeacherCourseContentPublicationStatus).mockRejectedValueOnce(failure);
      render(<TeacherCourseContentRoute courseId="9" />);

      await user.click(await screen.findByRole("button", { name: "Unpublish" }));

      expect(await screen.findByText(failure.message)).toBeVisible();
      cleanup();
    }

    await waitFor(() => expect(setTeacherCourseContentPublicationStatus).toHaveBeenCalledTimes(3));
  });
});
