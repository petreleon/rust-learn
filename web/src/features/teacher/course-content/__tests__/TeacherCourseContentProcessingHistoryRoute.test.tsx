import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { TeacherRequestError } from "@/lib/teacher";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import {
  loadTeacherContentProcessingHistory,
  loadTeacherCourseContentWorkspace,
} from "../api/courseContentApi";
import { TeacherCourseContentRoute } from "../route/TeacherCourseContentRoute";
import {
  courseContentSession,
  courseContentWorkspace,
  failedVideoContent,
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
  loadTeacherContentProcessingHistory: vi.fn(),
  loadTeacherCourseContentWorkspace: vi.fn(),
  processTeacherCourseContentItem: vi.fn(),
  requestTeacherContentUploadUrl: vi.fn(),
  setTeacherCourseContentPublicationStatus: vi.fn(),
  updateTeacherCourseContentItem: vi.fn(),
  uploadTeacherContentFile: vi.fn(),
}));

describe("TeacherCourseContentRoute processing history", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readBrowserSessionToken).mockReturnValue("teacher-token");
    vi.mocked(loadTeacherCourseContentWorkspace).mockResolvedValue({
      assessments: [],
      session: courseContentSession(),
      workspace: courseContentWorkspace([failedVideoContent()]),
    });
    vi.mocked(loadTeacherContentProcessingHistory).mockResolvedValue({
      content_id: 12,
      jobs: [{
        attempts: 2,
        created_at: "2026-06-16T08:00:00Z",
        id: 91,
        last_error: "transcode timed out",
        status: "failed",
        updated_at: "2026-06-16T08:01:00Z",
      }],
      object_key: "courses/9/intro.mp4",
    });
  });

  it("loads processing history for failed video content", async () => {
    const user = userEvent.setup();
    render(<TeacherCourseContentRoute courseId="9" />);

    expect(await screen.findByText("Transcode failed")).toBeVisible();
    await user.click(screen.getByRole("button", { name: "History" }));

    await waitFor(() =>
      expect(loadTeacherContentProcessingHistory).toHaveBeenCalledWith({
        chapterId: 3,
        contentId: 12,
        courseId: "9",
        token: "teacher-token",
      }),
    );
    expect(await screen.findByText("Processing history")).toBeVisible();
    expect(screen.getByText("courses/9/intro.mp4")).toBeVisible();
    expect(screen.getByText("transcode timed out")).toBeVisible();
  });

  it("shows backend errors from processing history inspection", async () => {
    const user = userEvent.setup();
    vi.mocked(loadTeacherContentProcessingHistory).mockRejectedValue(
      new TeacherRequestError("Processing history is unavailable.", 500, "server_error"),
    );
    render(<TeacherCourseContentRoute courseId="9" />);

    await user.click(await screen.findByRole("button", { name: "History" }));

    expect(await screen.findByText("Processing history is unavailable.")).toBeVisible();
  });
});
