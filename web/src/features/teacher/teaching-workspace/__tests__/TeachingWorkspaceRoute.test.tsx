import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import { TeachingWorkspaceRoute } from "../route/TeachingWorkspaceRoute";
import { createTeachingCourse, loadTeachingWorkspaceData } from "../api/teachingWorkspaceApi";
import {
  applicationSnapshot,
  teachingCourses,
  teachingSession,
} from "./teachingWorkspaceTestFixtures";

vi.mock("next/navigation", () => ({
  usePathname: () => "/teach",
  useRouter: () => ({ push: vi.fn() }),
}));

vi.mock("@/shared/session/browserSession", () => ({
  clearBrowserSession: vi.fn(),
  readBrowserSessionToken: vi.fn(),
}));

vi.mock("../api/teachingWorkspaceApi", () => ({
  createTeachingCourse: vi.fn(),
  loadTeachingWorkspaceData: vi.fn(),
}));

describe("TeachingWorkspaceRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readBrowserSessionToken).mockReturnValue("teacher-token");
    vi.mocked(loadTeachingWorkspaceData).mockResolvedValue({
      applicationSnapshot: applicationSnapshot(),
      courses: teachingCourses(),
      session: teachingSession(),
    });
    vi.mocked(createTeachingCourse).mockResolvedValue({
      description: null,
      id: 21,
      lifecycle_status: "draft",
      prerequisites: null,
      title: "Async Rust",
      topics: null,
    });
  });

  it("shows the signed-out state without loading teaching workspace APIs", async () => {
    vi.mocked(readBrowserSessionToken).mockReturnValue(null);

    render(<TeachingWorkspaceRoute view="dashboard" />);

    expect(await screen.findByRole("heading", { name: "Sign in required" })).toBeVisible();
    expect(loadTeachingWorkspaceData).not.toHaveBeenCalled();
  });

  it("loads dashboard data through the feature API boundary", async () => {
    render(<TeachingWorkspaceRoute view="dashboard" />);

    expect(await screen.findByRole("heading", { name: "Teaching workspace" })).toBeVisible();
    expect(await screen.findByRole("heading", { name: "Rust Safety" })).toBeVisible();
    await waitFor(() =>
      expect(loadTeachingWorkspaceData).toHaveBeenCalledWith({
        query: { lifecycleStatus: "all", search: "" },
        token: "teacher-token",
      }),
    );
  });

  it("applies course filters through the route controller", async () => {
    const user = userEvent.setup();
    render(<TeachingWorkspaceRoute view="courses" />);

    expect(await screen.findByRole("heading", { name: "Teaching courses" })).toBeVisible();
    expect(await screen.findByRole("heading", { name: "Rust Safety" })).toBeVisible();
    await user.type(screen.getByPlaceholderText("Course title"), "Rust");
    await user.selectOptions(screen.getByDisplayValue("All states"), "published");
    await user.click(screen.getByRole("button", { name: "Apply filters" }));

    await waitFor(() => expect(loadTeachingWorkspaceData).toHaveBeenCalledTimes(2));
    expect(loadTeachingWorkspaceData).toHaveBeenLastCalledWith({
      query: { lifecycleStatus: "published", search: "Rust" },
      token: "teacher-token",
    });
  });

  it("creates a draft course through the feature API boundary", async () => {
    const user = userEvent.setup();
    vi.mocked(loadTeachingWorkspaceData).mockResolvedValue({
      applicationSnapshot: applicationSnapshot(),
      courses: teachingCourses(),
      session: teachingSession({
        platform: {
          capabilities: [],
          delegated_permissions: [],
          direct_permissions: ["CREATE_COURSE"],
          effective_permissions: ["CREATE_COURSE"],
          roles: ["PLATFORM_ADMIN"],
        },
      }),
    });

    render(<TeachingWorkspaceRoute view="courses" />);

    await user.type(await screen.findByPlaceholderText("New course title"), "Async Rust");
    await user.click(screen.getByRole("button", { name: "Create draft course" }));

    await waitFor(() =>
      expect(createTeachingCourse).toHaveBeenCalledWith({
        draft: { targetValue: "platform", title: "Async Rust" },
        token: "teacher-token",
      }),
    );
    expect(await screen.findByText("Async Rust was created as draft.")).toBeVisible();
    expect(loadTeachingWorkspaceData).toHaveBeenCalledTimes(2);
  });

  it("clears stored session when signing out", async () => {
    const user = userEvent.setup();
    render(<TeachingWorkspaceRoute view="dashboard" />);

    expect(await screen.findByRole("heading", { name: "Teaching workspace" })).toBeVisible();
    expect(await screen.findByRole("heading", { name: "Rust Safety" })).toBeVisible();
    await user.click(screen.getAllByRole("button", { name: "Sign out" })[0]);

    expect(clearBrowserSession).toHaveBeenCalled();
    expect(await screen.findByRole("heading", { name: "Sign in required" })).toBeVisible();
  });
});
