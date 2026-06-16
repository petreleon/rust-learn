import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { TeacherRequestError } from "@/lib/teacher/TeacherRequestError";
import { loadCourseCompletionTerms } from "@/features/teacher/course-completion-terms/api/completionTermsApi";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import {
  loadTeacherCourseWorkspace,
  saveTeacherCourseLifecycle,
  saveTeacherCourseSettings,
} from "../api/courseWorkspaceApi";
import { TeacherCourseWorkspaceRoute } from "../route/TeacherCourseWorkspaceRoute";
import { courseWorkspace, courseWorkspaceSession } from "./courseWorkspaceTestFixtures";

vi.mock("next/navigation", () => ({
  usePathname: () => "/teach/courses/9",
  useRouter: () => ({ push: vi.fn() }),
}));

vi.mock("@/shared/session/browserSession", () => ({
  clearBrowserSession: vi.fn(),
  readBrowserSessionToken: vi.fn(),
}));

vi.mock("../api/courseWorkspaceApi", () => ({
  loadTeacherCourseWorkspace: vi.fn(),
  saveTeacherCourseLifecycle: vi.fn(),
  saveTeacherCourseSettings: vi.fn(),
}));

vi.mock("@/features/teacher/course-completion-terms/api/completionTermsApi", () => ({
  loadCourseCompletionTerms: vi.fn(),
}));

describe("TeacherCourseWorkspaceRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readBrowserSessionToken).mockReturnValue("teacher-token");
    vi.mocked(loadTeacherCourseWorkspace).mockResolvedValue({
      session: courseWorkspaceSession(),
      workspace: courseWorkspace(),
    });
    vi.mocked(loadCourseCompletionTerms).mockResolvedValue({
      active_terms: null,
      audit_events: [],
      terms: [],
    });
    vi.mocked(saveTeacherCourseLifecycle).mockResolvedValue({
      description: "Borrow checking fundamentals.",
      id: 9,
      lifecycle_status: "archived",
      prerequisites: "Rust basics",
      title: "Rust Safety",
      topics: "Ownership, borrowing",
    });
    vi.mocked(saveTeacherCourseSettings).mockResolvedValue({
      description: "Advanced ownership.",
      id: 9,
      lifecycle_status: "published",
      prerequisites: "Rust basics",
      title: "Advanced Rust Safety",
      topics: "Ownership, traits",
    });
  });

  it("shows the signed-out state without loading the workspace API", async () => {
    vi.mocked(readBrowserSessionToken).mockReturnValue(null);

    render(<TeacherCourseWorkspaceRoute courseId="9" />);

    expect(await screen.findByRole("heading", { name: "Sign in required" })).toBeVisible();
    expect(loadTeacherCourseWorkspace).not.toHaveBeenCalled();
  });

  it("loads course workspace data through the feature API boundary", async () => {
    render(<TeacherCourseWorkspaceRoute courseId="9" />);

    expect((await screen.findAllByRole("heading", { name: "Rust Safety" })).length).toBeGreaterThan(0);
    expect(screen.getByRole("heading", { name: "Course settings" })).toBeVisible();
    expect(screen.getByRole("heading", { name: "Completion terms" })).toBeVisible();
    expect(screen.getAllByRole("link", { name: "Open" })[0]).toHaveAttribute(
      "href",
      "/teach/courses/9/content",
    );
    await waitFor(() =>
      expect(loadTeacherCourseWorkspace).toHaveBeenCalledWith({
        courseId: "9",
        token: "teacher-token",
      }),
    );
  });

  it("saves course metadata through the feature API boundary", async () => {
    const user = userEvent.setup();
    render(<TeacherCourseWorkspaceRoute courseId="9" />);

    await user.clear(await screen.findByLabelText("Title"));
    await user.type(screen.getByLabelText("Title"), "Advanced Rust Safety");
    await user.clear(screen.getByLabelText("Description"));
    await user.type(screen.getByLabelText("Description"), "Advanced ownership.");
    await user.clear(screen.getByLabelText("Topics"));
    await user.type(screen.getByLabelText("Topics"), "Ownership, traits");
    await user.click(screen.getByRole("button", { name: "Save course settings" }));

    await waitFor(() =>
      expect(saveTeacherCourseSettings).toHaveBeenCalledWith({
        courseId: "9",
        draft: {
          description: "Advanced ownership.",
          prerequisites: "Rust basics",
          title: "Advanced Rust Safety",
          topics: "Ownership, traits",
        },
        token: "teacher-token",
      }),
    );
    expect(await screen.findByText("Course settings updated.")).toBeVisible();
  });

  it("submits lifecycle changes through the feature API boundary", async () => {
    const user = userEvent.setup();
    render(<TeacherCourseWorkspaceRoute courseId="9" />);

    await user.selectOptions(await screen.findByLabelText("Lifecycle"), "archived");
    await user.click(screen.getByRole("button", { name: "Update lifecycle" }));

    await waitFor(() =>
      expect(saveTeacherCourseLifecycle).toHaveBeenCalledWith({
        courseId: "9",
        status: "archived",
        token: "teacher-token",
      }),
    );
    expect(await screen.findByText("Course lifecycle updated.")).toBeVisible();
  });

  it("clears stored session when signing out", async () => {
    const user = userEvent.setup();
    render(<TeacherCourseWorkspaceRoute courseId="9" />);

    expect(await screen.findByRole("heading", { name: "Course settings" })).toBeVisible();
    await user.click(screen.getAllByRole("button", { name: "Sign out" })[0]);

    expect(clearBrowserSession).toHaveBeenCalled();
    expect(await screen.findByRole("heading", { name: "Sign in required" })).toBeVisible();
  });

  it("clears stored session after an unauthorized workspace load", async () => {
    vi.mocked(loadTeacherCourseWorkspace).mockRejectedValue(
      new TeacherRequestError("Teacher session expired", 401, "unauthorized"),
    );

    render(<TeacherCourseWorkspaceRoute courseId="9" />);

    expect((await screen.findAllByText("Teacher session expired")).length).toBeGreaterThan(0);
    expect(clearBrowserSession).toHaveBeenCalled();
  });

  it("shows a deleted-course message without clearing the session", async () => {
    vi.mocked(loadTeacherCourseWorkspace).mockRejectedValue(
      new TeacherRequestError("Course not found", 404, "course_not_found"),
    );

    render(<TeacherCourseWorkspaceRoute courseId="9" />);

    expect(
      (
        await screen.findAllByText(
          "This course no longer exists. It may have been deleted; return to your teaching courses before continuing.",
        )
      ).length,
    ).toBeGreaterThan(0);
    expect(clearBrowserSession).not.toHaveBeenCalled();
  });
});
