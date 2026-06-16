import { render, screen, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { TeacherCourseRewardsRoute } from "@/features/teacher/course-rewards/route/TeacherCourseRewardsRoute";
import { CourseCard } from "@/features/teacher/teaching-workspace/components/CourseCard";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type TeacherCourseDashboardItem } from "@/lib/teacher/TeacherCourseDashboardItem";
import { type TeacherCourseStudentsResponse } from "@/lib/teacher/TeacherCourseStudentsResponse";
import { TeacherRequestError } from "@/lib/teacher/TeacherRequestError";
import { readBrowserSessionToken } from "@/shared/session/browserSession";
import {
  loadCourseRewardCandidates,
  loadCourseRewardContext,
} from "@/features/teacher/course-rewards/api/courseRewardsApi";

vi.mock("next/navigation", () => ({
  usePathname: () => "/teach/courses/9/rewards",
  useRouter: () => ({ push: vi.fn() }),
}));

vi.mock("@/shared/session/browserSession", () => ({
  clearBrowserSession: vi.fn(),
  readBrowserSessionToken: vi.fn(),
}));

vi.mock("@/features/teacher/course-rewards/api/courseRewardsApi", () => ({
  decideCourseRewardCandidate: vi.fn(),
  loadCourseRewardCandidates: vi.fn(),
  loadCourseRewardContext: vi.fn(),
}));

const emptyScope = { capabilities: [], delegated_permissions: [], direct_permissions: [], effective_permissions: [], roles: [] };

function session(): CurrentSession {
  return {
    access: { learner: true, teacher: true, teacher_application: false, organization: false, platform_admin: false },
    courses: [{
      ...emptyScope,
      capabilities: [{ enabled: true, key: "teaching", label: "Teaching", permissions: [] }],
      id: 9,
      lifecycle_status: "published",
      title: "Rust Safety",
    }],
    delegated_permissions: [],
    organizations: [],
    platform: emptyScope,
    user: { email: "teacher@example.com", email_verified: true, id: 2, kyc_verified: true, name: "Teacher User" },
  };
}

function course(canViewRewards: boolean): TeacherCourseDashboardItem {
  return {
    content: { chapter_count: 1, content_count: 2, content_types: ["article"], has_content: true },
    description: null,
    id: 9,
    lifecycle_status: "published",
    organizations: [],
    prerequisites: null,
    permissions: {
      can_approve_reward_candidates: false,
      can_manage_content: true,
      can_manage_enrollments: true,
      can_manage_reward_rules: false,
      can_manage_settings: true,
      can_view_reward_candidates: canViewRewards,
    },
    reward_queue: { failed_count: 0, pending_teacher_count: 1, teacher_approved_count: 0 },
    rewards: { active_policy_count: 0, available: false, event_types: [], payment_strategies: [], token_amounts: [] },
    roster: { enrolled_student_count: 1, pending_join_request_count: 0, waitlisted_join_request_count: 0 },
    title: "Rust Safety",
    topics: null,
  };
}

function students(canViewRewards: boolean): TeacherCourseStudentsResponse {
  return {
    course: course(canViewRewards),
    progress_supported: true,
    reward_eligibility: { active_policy_count: 0, event_types: [], supported: false },
    reward_eligibility_supported: true,
    reward_evidence_supported: true,
    students: [],
    teacher_roles: ["TEACHER"],
    total: 1,
  };
}

describe("TeacherCourseRewardsRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readBrowserSessionToken).mockReturnValue("teacher-token");
    vi.mocked(loadCourseRewardContext).mockResolvedValue({
      session: session(),
      students: students(false),
    });
    vi.mocked(loadCourseRewardCandidates).mockResolvedValue([]);
  });

  it("shows the signed-out state without loading reward APIs", async () => {
    vi.mocked(readBrowserSessionToken).mockReturnValue(null);

    render(<TeacherCourseRewardsRoute courseId="9" />);

    expect(await screen.findByRole("heading", { name: "Sign in required" })).toBeVisible();
    expect(loadCourseRewardContext).not.toHaveBeenCalled();
    expect(loadCourseRewardCandidates).not.toHaveBeenCalled();
  });

  it("loads reward candidates through the feature API boundary", async () => {
    vi.mocked(loadCourseRewardContext).mockResolvedValue({
      session: session(),
      students: students(true),
    });

    render(<TeacherCourseRewardsRoute courseId="9" />);

    expect(await screen.findByRole("heading", { name: "No matching candidates" })).toBeVisible();
    expect(loadCourseRewardContext).toHaveBeenCalledWith({
      courseId: "9",
      token: "teacher-token",
    });
    expect(loadCourseRewardCandidates).toHaveBeenCalledWith({
      courseId: "9",
      status: "pending_teacher_approval",
      token: "teacher-token",
    });
  });

  it("keeps course shell context when reward review permission is missing", async () => {
    render(<TeacherCourseRewardsRoute courseId="9" />);

    expect((await screen.findAllByText("Reward review requires course reward candidate permission for this course.")).length).toBeGreaterThan(0);
    expect(screen.getAllByText("Rust Safety").length).toBeGreaterThan(0);
    expect(screen.getAllByText("Teacher User").length).toBeGreaterThan(0);
    expect(screen.getByRole("link", { name: "Course workspace" })).toHaveAttribute("href", "/teach/courses/9");
    expect(loadCourseRewardCandidates).not.toHaveBeenCalled();
  });

  it("keeps course shell context when the reward API denies direct navigation", async () => {
    vi.mocked(loadCourseRewardContext).mockResolvedValue({
      session: session(),
      students: students(true),
    });
    vi.mocked(loadCourseRewardCandidates).mockRejectedValue(
      new TeacherRequestError("User does not have reward candidate permission", 403, "permission_denied"),
    );

    render(<TeacherCourseRewardsRoute courseId="9" />);

    expect((await screen.findAllByText("User does not have reward candidate permission")).length).toBeGreaterThan(0);
    await waitFor(() => expect(loadCourseRewardCandidates).toHaveBeenCalled());
    expect(screen.getAllByText("Rust Safety").length).toBeGreaterThan(0);
    expect(screen.getAllByText("Teacher User").length).toBeGreaterThan(0);
  });
});

describe("teacher reward action links", () => {
  it("disables the course-card reward action when reward permission is missing", () => {
    render(<CourseCard course={course(false)} />);

    expect(screen.queryByRole("link", { name: "Rewards" })).not.toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Rewards" })).toBeDisabled();
    expect(screen.getByRole("button", { name: "Rewards" })).toHaveAttribute("title", "Reward candidate permission required");
  });

  it("links the course-card reward action when reward permission is present", () => {
    render(<CourseCard course={course(true)} />);

    expect(screen.getByRole("link", { name: "Rewards" })).toHaveAttribute("href", "/teach/courses/9/rewards");
  });
});
