import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type TeacherApplication } from "@/lib/teacher/TeacherApplication";
import { type TeacherApplicationSnapshot } from "@/lib/teacher/TeacherApplicationSnapshot";
import { clearBrowserSession, readBrowserSessionToken } from "@/shared/session/browserSession";
import { TeacherApplicationRoute } from "../route/TeacherApplicationRoute";
import {
  loadTeacherApplicationData,
  loadTeacherApplicationSnapshot,
  submitTeacherApplicationDraft,
} from "../api/teacherApplicationApi";

vi.mock("next/navigation", () => ({
  usePathname: () => "/teach/apply",
  useRouter: () => ({ push: vi.fn() }),
}));

vi.mock("@/shared/session/browserSession", () => ({
  clearBrowserSession: vi.fn(),
  readBrowserSessionToken: vi.fn(),
}));

vi.mock("../api/teacherApplicationApi", () => ({
  loadTeacherApplicationData: vi.fn(),
  loadTeacherApplicationSnapshot: vi.fn(),
  submitTeacherApplicationDraft: vi.fn(),
}));

describe("TeacherApplicationRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    window.sessionStorage.clear();
    vi.mocked(readBrowserSessionToken).mockReturnValue("teacher-token");
    vi.mocked(loadTeacherApplicationData).mockResolvedValue({ session: teacherApplicationSession(), snapshot: teacherApplicationSnapshot(null) });
    vi.mocked(loadTeacherApplicationSnapshot).mockResolvedValue(teacherApplicationSnapshot(teacherApplication()));
    vi.mocked(submitTeacherApplicationDraft).mockResolvedValue(undefined);
  });

  it("shows the signed-out state without loading teacher application APIs", async () => {
    vi.mocked(readBrowserSessionToken).mockReturnValue(null);

    render(<TeacherApplicationRoute />);

    expect(await screen.findByRole("heading", { name: "Sign in required" })).toBeVisible();
    expect(loadTeacherApplicationData).not.toHaveBeenCalled();
  });

  it("loads application state through the feature API boundary", async () => {
    render(<TeacherApplicationRoute />);

    expect(await screen.findByRole("heading", { name: "Ready to apply" })).toBeVisible();
    expect(await screen.findByRole("heading", { name: "Application details" })).toBeVisible();
    await waitFor(() =>
      expect(loadTeacherApplicationData).toHaveBeenCalledWith({
        token: "teacher-token",
      }),
    );
  });

  it("submits a valid platform application and refreshes the snapshot", async () => {
    const user = userEvent.setup();
    render(<TeacherApplicationRoute />);

    expect(await screen.findByRole("heading", { name: "Application details" })).toBeVisible();
    await user.type(screen.getByLabelText("Experience summary"), "I teach Rust ownership through project reviews.");
    await user.type(screen.getByPlaceholderText("One link per line"), "https://example.com/teaching");
    await user.click(screen.getByRole("button", { name: "Submit application" }));

    await waitFor(() =>
      expect(submitTeacherApplicationDraft).toHaveBeenCalledWith({
        draft: expect.objectContaining({
          experienceSummary: "I teach Rust ownership through project reviews.",
          portfolioLinks: "https://example.com/teaching",
          requestedScope: "platform",
        }),
        portfolioLinks: ["https://example.com/teaching"],
        token: "teacher-token",
      }),
    );
    expect(loadTeacherApplicationSnapshot).toHaveBeenCalledWith({ token: "teacher-token" });
    expect(await screen.findByText("Application submitted")).toBeVisible();
  });

  it("clears stored session when signing out", async () => {
    const user = userEvent.setup();
    render(<TeacherApplicationRoute />);

    expect(await screen.findByRole("heading", { name: "Ready to apply" })).toBeVisible();
    await user.click(screen.getAllByRole("button", { name: "Sign out" })[0]);

    expect(clearBrowserSession).toHaveBeenCalled();
    expect(await screen.findByRole("heading", { name: "Sign in required" })).toBeVisible();
  });
});

function teacherApplicationSession(): CurrentSession {
  return {
    access: { learner: true, organization: false, platform_admin: false, teacher: false, teacher_application: true },
    courses: [
      {
        capabilities: [],
        delegated_permissions: [],
        direct_permissions: [],
        effective_permissions: [],
        id: 7,
        lifecycle_status: "published",
        roles: [],
        title: "Rust Foundations",
      },
    ],
    delegated_permissions: [],
    organizations: [
      {
        capabilities: [],
        delegated_permissions: [],
        direct_permissions: [],
        effective_permissions: [],
        id: 3,
        name: "Rust Org",
        roles: [],
      },
    ],
    platform: {
      capabilities: [],
      delegated_permissions: [],
      direct_permissions: ["APPLY_TEACHER"],
      effective_permissions: ["APPLY_TEACHER"],
      roles: [],
    },
    user: {
      email: "ada@example.com",
      email_verified: true,
      id: 11,
      kyc_verified: true,
      name: "Ada Teacher",
    },
  };
}

function teacherApplicationSnapshot(application: TeacherApplication | null): TeacherApplicationSnapshot {
  return {
    application,
    audit_events: [],
  };
}

function teacherApplication(overrides: Partial<TeacherApplication> = {}): TeacherApplication {
  return {
    applicant_user_id: 11,
    created_at: "2026-06-12T10:00:00Z",
    decided_at: null,
    decision_reason: null,
    experience_summary: "I teach Rust ownership through project reviews.",
    id: 31,
    idempotency_key: "application-key",
    organization_sponsor_id: null,
    portfolio_links: ["https://example.com/teaching"],
    requested_course_id: null,
    requested_organization_id: null,
    requested_scope: "platform",
    reviewer_id: null,
    status: "submitted",
    updated_at: "2026-06-12T10:00:00Z",
    ...overrides,
  };
}
