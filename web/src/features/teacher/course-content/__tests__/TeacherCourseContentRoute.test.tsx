import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import {
  createTeacherCourseContentItem,
  loadTeacherCourseContentWorkspace,
  processTeacherCourseContentItem,
  requestTeacherContentUploadUrl,
  uploadTeacherContentFile,
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
  loadTeacherCourseContentWorkspace: vi.fn(),
  processTeacherCourseContentItem: vi.fn(),
  requestTeacherContentUploadUrl: vi.fn(),
  updateTeacherCourseContentItem: vi.fn(),
  uploadTeacherContentFile: vi.fn(),
}));

describe("TeacherCourseContentRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readBrowserSessionToken).mockReturnValue("teacher-token");
    vi.mocked(loadTeacherCourseContentWorkspace).mockResolvedValue({
      session: courseContentSession(),
      workspace: courseContentWorkspace(),
    });
    vi.mocked(createTeacherCourseContentItem).mockResolvedValue({
      chapter_id: 3,
      content_type: "article",
      data: "New lesson body",
      id: 21,
      order: 1,
    });
    vi.mocked(processTeacherCourseContentItem).mockResolvedValue({ message: "Processing queued." });
    vi.mocked(requestTeacherContentUploadUrl).mockResolvedValue({
      object_key: "courses/9/intro.mp4",
      upload_url: "https://upload.example.test/intro.mp4",
    });
    vi.mocked(uploadTeacherContentFile).mockImplementation(async ({ onProgress }) => {
      onProgress(35);
      onProgress(100);
    });
  });

  it("shows signed-out state without loading content APIs", async () => {
    vi.mocked(readBrowserSessionToken).mockReturnValue(null);

    render(<TeacherCourseContentRoute courseId="9" />);

    expect(await screen.findByRole("heading", { name: "Sign in required" })).toBeVisible();
    expect(loadTeacherCourseContentWorkspace).not.toHaveBeenCalled();
  });

  it("loads content authoring through the feature API boundary", async () => {
    render(<TeacherCourseContentRoute courseId="9" />);

    expect(await screen.findByRole("heading", { name: "Content authoring" })).toBeVisible();
    expect(screen.getByRole("heading", { name: "Create text content" })).toBeVisible();
    await waitFor(() =>
      expect(loadTeacherCourseContentWorkspace).toHaveBeenCalledWith({
        courseId: "9",
        token: "teacher-token",
      }),
    );
  });

  it("creates text content from the course content route", async () => {
    const user = userEvent.setup();
    render(<TeacherCourseContentRoute courseId="9" />);

    await user.type(await screen.findByLabelText("Lesson body"), "New lesson body");
    await user.click(screen.getByRole("button", { name: "Create content" }));

    await waitFor(() =>
      expect(createTeacherCourseContentItem).toHaveBeenCalledWith({
        chapterId: "3",
        courseId: "9",
        payload: { content_type: "article", data: "New lesson body", order: 1 },
        token: "teacher-token",
      }),
    );
    expect(await screen.findByText("Content item created.")).toBeVisible();
  });

  it("shows upload progress while creating file content", async () => {
    const user = userEvent.setup();
    render(<TeacherCourseContentRoute courseId="9" />);

    await user.selectOptions(await screen.findByLabelText("Kind"), "file");
    await user.upload(screen.getByLabelText("File"), new File(["video"], "intro.mp4", { type: "video/mp4" }));
    await user.click(screen.getByRole("button", { name: "Upload content" }));

    await waitFor(() => expect(screen.getByText("100% uploaded")).toBeVisible());
    expect(requestTeacherContentUploadUrl).toHaveBeenCalledWith({
      chapterId: "3",
      contentType: "video/mp4",
      courseId: "9",
      filename: "intro.mp4",
      token: "teacher-token",
    });
    expect(uploadTeacherContentFile).toHaveBeenCalled();
  });

  it("queues processing retry for failed uploaded content", async () => {
    const user = userEvent.setup();
    vi.mocked(loadTeacherCourseContentWorkspace).mockResolvedValue({
      session: courseContentSession(),
      workspace: courseContentWorkspace([failedVideoContent()]),
    });

    render(<TeacherCourseContentRoute courseId="9" />);

    expect(await screen.findByText("Transcode failed")).toBeVisible();
    await user.click(screen.getByRole("button", { name: "Process" }));

    await waitFor(() =>
      expect(processTeacherCourseContentItem).toHaveBeenCalledWith({
        chapterId: 3,
        contentId: 12,
        courseId: "9",
        token: "teacher-token",
      }),
    );
    expect(await screen.findByText("Processing queued.")).toBeVisible();
  });
});
