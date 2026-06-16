import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import { TeachingWorkspaceRoute } from "../route/TeachingWorkspaceRoute";
import { loadTeachingWorkspaceData } from "../api/teachingWorkspaceApi";
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
