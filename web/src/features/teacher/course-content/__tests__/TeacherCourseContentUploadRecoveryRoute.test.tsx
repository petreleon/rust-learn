import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { TeacherRequestError } from "@/lib/teacher";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import {
  loadTeacherCourseContentWorkspace,
  requestTeacherContentUploadUrl,
  uploadTeacherContentFile,
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
  updateTeacherCourseContentItem: vi.fn(),
  uploadTeacherContentFile: vi.fn(),
}));

describe("TeacherCourseContentRoute upload recovery", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readBrowserSessionToken).mockReturnValue("teacher-token");
    vi.mocked(loadTeacherCourseContentWorkspace).mockResolvedValue({
      assessments: [],
      session: courseContentSession(),
      workspace: courseContentWorkspace(),
    });
    vi.mocked(requestTeacherContentUploadUrl).mockResolvedValue({
      object_key: "courses/9/chapters/3/intro.mp4",
      upload_url: "https://upload.example.test/intro.mp4",
    });
  });

  it("shows an actionable message when a presigned upload URL expires", async () => {
    const user = userEvent.setup();
    vi.mocked(uploadTeacherContentFile).mockRejectedValue(
      new TeacherRequestError(
        "Upload URL expired or was rejected. Select Upload content again to request a fresh URL.",
        403,
        "upload_url_expired",
      ),
    );
    render(<TeacherCourseContentRoute courseId="9" />);

    await user.selectOptions(await screen.findByLabelText("Kind"), "file");
    await user.upload(screen.getByLabelText("File"), new File(["video"], "intro.mp4", { type: "video/mp4" }));
    await user.click(screen.getByRole("button", { name: "Upload content" }));

    expect(await screen.findByText(/Upload URL expired or was rejected/)).toBeVisible();
    expect(requestTeacherContentUploadUrl).toHaveBeenCalledWith({
      chapterId: "3",
      contentType: "video/mp4",
      courseId: "9",
      filename: "intro.mp4",
      token: "teacher-token",
    });
  });
});
