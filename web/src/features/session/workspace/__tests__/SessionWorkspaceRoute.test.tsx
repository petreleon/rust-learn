import { render, screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { SessionRequestError } from "@/lib/session/SessionRequestError";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import { loadSessionWorkspace } from "../api/sessionWorkspaceApi";
import { SessionWorkspaceRoute } from "../route/SessionWorkspaceRoute";

vi.mock("next/navigation", () => ({
  usePathname: () => "/session",
  useRouter: () => ({ push: vi.fn() }),
}));

vi.mock("@/shared/session/browserSession", () => ({
  clearBrowserSession: vi.fn(),
  readBrowserSessionToken: vi.fn(),
}));

vi.mock("../api/sessionWorkspaceApi", () => ({
  loadSessionWorkspace: vi.fn(),
}));

describe("SessionWorkspaceRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readBrowserSessionToken).mockReturnValue("session-token");
    vi.mocked(loadSessionWorkspace).mockResolvedValue(sessionFixture());
  });

  it("shows the signed-out state without loading the session API", async () => {
    vi.mocked(readBrowserSessionToken).mockReturnValue(null);

    render(<SessionWorkspaceRoute />);

    expect(await screen.findByRole("heading", { name: "Sign in required" })).toBeVisible();
    expect(loadSessionWorkspace).not.toHaveBeenCalled();
  });

  it("loads the current session through the feature API boundary", async () => {
    render(<SessionWorkspaceRoute />);

    expect(await screen.findByRole("heading", { name: "Current session" })).toBeVisible();
    expect((await screen.findAllByText("Ada Session"))[0]).toBeVisible();
    expect((await screen.findAllByText("Rust Org"))[0]).toBeVisible();
    await waitFor(() => expect(loadSessionWorkspace).toHaveBeenCalledWith({ token: "session-token" }));
  });

  it("refreshes the session through the controller", async () => {
    const user = userEvent.setup();
    render(<SessionWorkspaceRoute />);

    expect((await screen.findAllByText("Ada Session"))[0]).toBeVisible();
    await user.click(within(screen.getByLabelText("Workspace status")).getByRole("button", { name: "Refresh" }));

    await waitFor(() => expect(loadSessionWorkspace).toHaveBeenCalledTimes(2));
  });

  it("clears expired stored sessions", async () => {
    vi.mocked(loadSessionWorkspace).mockRejectedValue(new SessionRequestError("Expired session", 401, "unauthorized"));

    render(<SessionWorkspaceRoute />);

    expect(await screen.findByText("Session expired")).toBeVisible();
    expect(await screen.findByText("Expired session")).toBeVisible();
    expect(clearBrowserSession).toHaveBeenCalled();
  });

  it("clears stored session when signing out", async () => {
    const user = userEvent.setup();
    render(<SessionWorkspaceRoute />);

    expect((await screen.findAllByText("Ada Session"))[0]).toBeVisible();
    await user.click(screen.getAllByRole("button", { name: "Sign out" })[0]);

    expect(clearBrowserSession).toHaveBeenCalled();
    expect(await screen.findByRole("heading", { name: "Sign in required" })).toBeVisible();
  });
});

function sessionFixture(): CurrentSession {
  return {
    access: { learner: true, organization: true, platform_admin: false, teacher: true, teacher_application: true },
    courses: [
      {
        capabilities: [],
        delegated_permissions: [],
        direct_permissions: ["MANAGE_COURSE_CONTENT"],
        effective_permissions: ["MANAGE_COURSE_CONTENT"],
        id: 7,
        lifecycle_status: "published",
        roles: ["teacher"],
        title: "Rust Foundations",
      },
    ],
    delegated_permissions: [
      {
        course_id: 7,
        course_lifecycle_status: "published",
        course_title: "Rust Foundations",
        expires_at: "2026-06-20T10:00:00Z",
        grantor_user_id: 1,
        id: 12,
        organization_id: null,
        organization_name: null,
        permission: "VIEW_REWARD_AUDIT",
        scope_type: "course",
      },
    ],
    organizations: [
      {
        capabilities: [],
        delegated_permissions: [],
        direct_permissions: ["VIEW_ORGANIZATION"],
        effective_permissions: ["VIEW_ORGANIZATION"],
        id: 3,
        name: "Rust Org",
        roles: ["organization_admin"],
      },
    ],
    platform: {
      capabilities: [],
      delegated_permissions: ["VIEW_REWARD_AUDIT"],
      direct_permissions: [],
      effective_permissions: ["VIEW_REWARD_AUDIT"],
      roles: [],
    },
    user: {
      email: "ada@example.com",
      email_verified: true,
      id: 11,
      kyc_verified: false,
      name: "Ada Session",
    },
  };
}
