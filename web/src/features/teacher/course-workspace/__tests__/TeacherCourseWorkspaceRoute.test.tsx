import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { TeacherRequestError } from "@/lib/teacher/TeacherRequestError";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import { loadTeacherCourseWorkspace } from "../api/courseWorkspaceApi";
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
}));

describe("TeacherCourseWorkspaceRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readBrowserSessionToken).mockReturnValue("teacher-token");
    vi.mocked(loadTeacherCourseWorkspace).mockResolvedValue({
      session: courseWorkspaceSession(),
      workspace: courseWorkspace(),
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
    expect(screen.getByRole("heading", { name: "Ownership" })).toBeVisible();
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

  it("clears stored session when signing out", async () => {
    const user = userEvent.setup();
    render(<TeacherCourseWorkspaceRoute courseId="9" />);

    expect(await screen.findByRole("heading", { name: "Ownership" })).toBeVisible();
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
});
