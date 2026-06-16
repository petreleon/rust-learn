import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { clearStoredSessionToken } from "@/lib/session/clearStoredSessionToken";
import { readStoredSessionToken } from "@/lib/session/readStoredSessionToken";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type TeacherCourseDashboardItem } from "@/lib/teacher/TeacherCourseDashboardItem";
import { type TeacherCourseEnrollmentWorkspaceResponse } from "@/lib/teacher/TeacherCourseEnrollmentWorkspaceResponse";
import { TeacherCourseEnrollmentsRoute } from "../route/TeacherCourseEnrollmentsRoute";
import {
  decideEnrollmentRequest,
  loadTeacherCourseEnrollments,
} from "../api/enrollmentApi";

vi.mock("next/navigation", () => ({
  usePathname: () => "/teach/courses/9/enrollments",
  useRouter: () => ({ push: vi.fn() }),
}));

vi.mock("@/lib/session/readStoredSessionToken", () => ({ readStoredSessionToken: vi.fn() }));
vi.mock("@/lib/session/clearStoredSessionToken", () => ({ clearStoredSessionToken: vi.fn() }));
vi.mock("../api/enrollmentApi", () => ({
  decideEnrollmentRequest: vi.fn(),
  loadTeacherCourseEnrollments: vi.fn(),
  removeEnrollmentLearner: vi.fn(),
}));

const emptyScope = { capabilities: [], delegated_permissions: [], direct_permissions: [], effective_permissions: [], roles: [] };

describe("TeacherCourseEnrollmentsRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readStoredSessionToken).mockReturnValue("teacher-token");
    vi.mocked(loadTeacherCourseEnrollments).mockResolvedValue({
      session: sessionFixture(),
      workspace: workspaceFixture(),
    });
    vi.mocked(decideEnrollmentRequest).mockResolvedValue({
      course_id: 9, created_at: "2026-06-01T10:00:00Z", decided_at: "2026-06-01T11:00:00Z",
      decision_reason: null, id: 55, requester_user_id: 7, reviewer_user_id: 2, status: "approved",
      updated_at: "2026-06-01T11:00:00Z",
    });
  });

  it("shows the signed-out state without loading the workflow", async () => {
    vi.mocked(readStoredSessionToken).mockReturnValue(null);

    render(<TeacherCourseEnrollmentsRoute courseId="9" />);

    expect(await screen.findByRole("heading", { name: "Sign in required" })).toBeVisible();
    expect(loadTeacherCourseEnrollments).not.toHaveBeenCalled();
  });

  it("loads the enrollment workspace through the feature API", async () => {
    render(<TeacherCourseEnrollmentsRoute courseId="9" />);

    expect(await screen.findByRole("heading", { name: "Enrollment queue" })).toBeVisible();
    expect(screen.getByText("Ada Learner")).toBeVisible();
    expect(screen.getByText("Grace Student")).toBeVisible();
    expect(loadTeacherCourseEnrollments).toHaveBeenCalledWith({
      courseId: "9",
      status: "open",
      token: "teacher-token",
    });
  });

  it("submits a join-request decision through the controller", async () => {
    const user = userEvent.setup();
    render(<TeacherCourseEnrollmentsRoute courseId="9" />);

    await screen.findByRole("heading", { name: "Enrollment queue" });
    await user.click(screen.getByRole("button", { name: "Apply decision" }));

    await waitFor(() => expect(decideEnrollmentRequest).toHaveBeenCalled());
    expect(decideEnrollmentRequest).toHaveBeenCalledWith({
      courseId: "9",
      payload: { decision_reason: null, status: "approved" },
      requestId: 55,
      token: "teacher-token",
    });
    expect(await screen.findByText("Enrollment request updated.")).toBeVisible();
  });

  it("clears stored session when signing out", async () => {
    const user = userEvent.setup();
    render(<TeacherCourseEnrollmentsRoute courseId="9" />);

    await screen.findByRole("heading", { name: "Enrollment queue" });
    await user.click(screen.getAllByRole("button", { name: "Sign out" })[0]);

    expect(clearStoredSessionToken).toHaveBeenCalled();
    expect(await screen.findByRole("heading", { name: "Sign in required" })).toBeVisible();
  });
});

function sessionFixture(): CurrentSession {
  return {
    access: { learner: true, organization: false, platform_admin: false, teacher: true, teacher_application: false },
    courses: [{ ...emptyScope, id: 9, lifecycle_status: "published", title: "Rust Safety" }],
    delegated_permissions: [],
    organizations: [],
    platform: emptyScope,
    user: { email: "teacher@example.com", email_verified: true, id: 2, kyc_verified: true, name: "Teacher User" },
  };
}

function workspaceFixture(): TeacherCourseEnrollmentWorkspaceResponse {
  return {
    course: courseFixture(),
    join_requests: {
      limit: 25,
      offset: 0,
      requests: [{
        can_decide: true,
        created_at: "2026-06-01T10:00:00Z",
        decided_at: null,
        decision_reason: null,
        id: 55,
        requester: userFixture(7, "Ada Learner"),
        reviewer: null,
        status: "pending",
        updated_at: "2026-06-01T10:00:00Z",
      }],
      status: "open",
      total: 1,
    },
    progress_supported: true,
    reward_eligibility: { active_policy_count: 1, event_types: ["lesson_completed"], supported: true },
    reward_eligibility_supported: true,
    roster: {
      learners: [{
        access_state: "active",
        can_remove: true,
        latest_join_request_status: "approved",
        progress_supported: true,
        reward_eligibility: {
          active_policy_count: 1,
          event_types: ["lesson_completed"],
          reward_candidate_count: 0,
          supported: true,
        },
        reward_eligibility_supported: true,
        roles: ["LEARNER"],
        user: userFixture(8, "Grace Student"),
      }],
      total: 1,
    },
    teacher_roles: ["TEACHER"],
  };
}

function courseFixture(): TeacherCourseDashboardItem {
  return {
    content: { chapter_count: 1, content_count: 2, content_types: ["article"], has_content: true },
    id: 9,
    lifecycle_status: "published",
    organizations: [],
    permissions: {
      can_approve_reward_candidates: false,
      can_manage_content: true,
      can_manage_enrollments: true,
      can_manage_reward_rules: false,
      can_manage_settings: true,
      can_view_reward_candidates: false,
    },
    reward_queue: { failed_count: 0, pending_teacher_count: 0, teacher_approved_count: 0 },
    rewards: { active_policy_count: 1, available: true, event_types: [], payment_strategies: [], token_amounts: [] },
    roster: { enrolled_student_count: 1, pending_join_request_count: 1, waitlisted_join_request_count: 0 },
    title: "Rust Safety",
  };
}

function userFixture(id: number, name: string) {
  return { email: `${name.toLowerCase().replace(" ", ".")}@example.com`, email_verified: true, id, kyc_verified: true, name };
}
