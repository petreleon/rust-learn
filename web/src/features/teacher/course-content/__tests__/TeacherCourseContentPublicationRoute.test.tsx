import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
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

describe("TeacherCourseContentRoute publication", () => {
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

  it("unpublishes content from the course content route", async () => {
    const user = userEvent.setup();
    render(<TeacherCourseContentRoute courseId="9" />);

    await user.click(await screen.findByRole("button", { name: "Unpublish" }));

    await waitFor(() =>
      expect(setTeacherCourseContentPublicationStatus).toHaveBeenCalledWith({
        chapterId: 3,
        contentId: 11,
        courseId: "9",
        publicationStatus: "unpublished",
        token: "teacher-token",
      }),
    );
    expect(await screen.findByText("Content item unpublished.")).toBeVisible();
  });
});
