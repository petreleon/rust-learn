import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { AdminRequestError } from "@/lib/admin/AdminRequestError";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import { AdminTeacherApplicationsRoute } from "../route/AdminTeacherApplicationsRoute";
import {
  decideAdminTeacherApplication,
  loadAdminTeacherApplicationAudit,
  loadAdminTeacherApplications,
  loadAdminTeacherApplicationSession,
} from "../api/teacherApplicationsApi";
import {
  adminTeacherApplicationSession,
  teacherApplication,
  teacherApplicationAuditEvent,
  teacherApplicationDecisionResult,
  teacherApplicationResponse,
} from "./adminTeacherApplicationsTestFixtures";

vi.mock("next/navigation", () => ({
  usePathname: () => "/admin/teacher-applications",
  useRouter: () => ({ push: vi.fn() }),
}));

vi.mock("@/shared/session/browserSession", () => ({
  clearBrowserSession: vi.fn(),
  readBrowserSessionToken: vi.fn(),
}));

vi.mock("../api/teacherApplicationsApi", () => ({
  decideAdminTeacherApplication: vi.fn(),
  loadAdminTeacherApplicationAudit: vi.fn(),
  loadAdminTeacherApplications: vi.fn(),
  loadAdminTeacherApplicationSession: vi.fn(),
}));

function mockToken(token: string | null) {
  vi.mocked(readBrowserSessionToken).mockReturnValue(token);
}

describe("AdminTeacherApplicationsRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockToken("admin-token");
    vi.mocked(loadAdminTeacherApplicationSession).mockResolvedValue(
      adminTeacherApplicationSession([
        "REVIEW_TEACHER_APPLICATIONS",
        "APPROVE_TEACHER_APPLICATION",
        "REJECT_TEACHER_APPLICATION",
      ]),
    );
    vi.mocked(loadAdminTeacherApplications).mockResolvedValue(teacherApplicationResponse([]));
    vi.mocked(loadAdminTeacherApplicationAudit).mockResolvedValue([]);
  });

  it("shows the signed-out state without loading teacher application APIs", async () => {
    mockToken(null);

    render(<AdminTeacherApplicationsRoute />);

    expect(await screen.findByText("Sign in required")).toBeVisible();
    expect(loadAdminTeacherApplicationSession).not.toHaveBeenCalled();
    expect(loadAdminTeacherApplications).not.toHaveBeenCalled();
  });

  it("loads teacher applications and selected application audit", async () => {
    vi.mocked(loadAdminTeacherApplications).mockResolvedValue(
      teacherApplicationResponse([teacherApplication()]),
    );
    vi.mocked(loadAdminTeacherApplicationAudit).mockResolvedValue([teacherApplicationAuditEvent()]);

    render(<AdminTeacherApplicationsRoute />);

    expect((await screen.findAllByText("Ada Teacher"))[0]).toBeVisible();
    await waitFor(() =>
      expect(loadAdminTeacherApplications).toHaveBeenCalledWith({
        offset: 0,
        search: "",
        status: "submitted",
        token: "admin-token",
      }),
    );
    await waitFor(() =>
      expect(loadAdminTeacherApplicationAudit).toHaveBeenCalledWith({
        applicationId: 41,
        token: "admin-token",
      }),
    );
    expect(await screen.findByText("Initial submission")).toBeVisible();
  });

  it("approves the selected teacher application", async () => {
    const approved = teacherApplication({
      decided_at: "2026-06-12T11:00:00Z",
      decision_reason: "Ready to teach",
      reviewer: { email: "admin@example.com", id: 1, name: "Admin User" },
      status: "approved",
    });
    vi.mocked(loadAdminTeacherApplications)
      .mockResolvedValueOnce(teacherApplicationResponse([teacherApplication()]))
      .mockResolvedValueOnce(teacherApplicationResponse([approved]));
    vi.mocked(decideAdminTeacherApplication).mockResolvedValue(teacherApplicationDecisionResult());
    const user = userEvent.setup();

    render(<AdminTeacherApplicationsRoute />);

    expect((await screen.findAllByText("Ada Teacher"))[0]).toBeVisible();
    await user.selectOptions(screen.getByLabelText("Decision status"), "approved");
    await user.type(screen.getByLabelText("Decision reason"), "Ready to teach");
    await user.click(screen.getByRole("button", { name: "Save decision" }));

    await waitFor(() =>
      expect(decideAdminTeacherApplication).toHaveBeenCalledWith({
        applicationId: 41,
        decisionReason: "Ready to teach",
        status: "approved",
        token: "admin-token",
      }),
    );
    expect(await screen.findByText("Decision saved")).toBeVisible();
    expect(await screen.findByText("Approved and rejected applications are final in the current backend contract.")).toBeVisible();
  });

  it("refreshes application data when the decision conflicts", async () => {
    const changed = teacherApplication({ status: "needs_changes" });
    vi.mocked(loadAdminTeacherApplications)
      .mockResolvedValueOnce(teacherApplicationResponse([teacherApplication()]))
      .mockResolvedValueOnce(teacherApplicationResponse([changed]));
    vi.mocked(decideAdminTeacherApplication).mockRejectedValue(
      new AdminRequestError("Application changed", 409, "conflict"),
    );
    const user = userEvent.setup();

    render(<AdminTeacherApplicationsRoute />);

    expect((await screen.findAllByText("Ada Teacher"))[0]).toBeVisible();
    await user.type(screen.getByLabelText("Decision reason"), "Needs stronger portfolio");
    await user.click(screen.getByRole("button", { name: "Save decision" }));

    expect((await screen.findAllByText("Application changed"))[0]).toBeVisible();
    await waitFor(() => expect(loadAdminTeacherApplications).toHaveBeenCalledTimes(2));
  });

  it("shows a permission gate before calling the teacher application API", async () => {
    vi.mocked(loadAdminTeacherApplicationSession).mockResolvedValue(
      adminTeacherApplicationSession(["VIEW_REPORT"]),
    );

    render(<AdminTeacherApplicationsRoute />);

    expect(await screen.findByText("Teacher application review unavailable")).toBeVisible();
    expect(loadAdminTeacherApplications).not.toHaveBeenCalled();
  });
});
