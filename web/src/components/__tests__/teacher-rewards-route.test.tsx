import { render, screen, waitFor } from "@testing-library/react";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { CourseCard } from "@/features/teacher/teaching-workspace/components/CourseCard";
import { TeacherCourseRewardsRoute } from "@/components/teacher-routes/TeacherCourseRewardsRoute";
import { fetchCurrentSession, readStoredSessionToken, type CurrentSession } from "@/lib/session";
import {
  fetchTeacherRewardCandidates,
  fetchTeachingCourseStudents,
  TeacherRequestError,
  type TeacherCourseDashboardItem,
  type TeacherCourseStudentsResponse,
} from "@/lib/teacher";

vi.mock("next/navigation", () => ({
  usePathname: () => "/teach/courses/9/rewards",
  useRouter: () => ({ push: vi.fn() }),
}));

vi.mock("@/lib/session", async (importOriginal) => {
  const actual = await importOriginal<typeof import("@/lib/session")>();
  return {
    ...actual,
    clearStoredSessionToken: vi.fn(),
    fetchCurrentSession: vi.fn(),
    readStoredSessionToken: vi.fn(),
  };
});

vi.mock("@/lib/teacher", async (importOriginal) => {
  const actual = await importOriginal<typeof import("@/lib/teacher")>();
  return {
    ...actual,
    fetchTeacherRewardCandidates: vi.fn(),
    fetchTeachingCourseStudents: vi.fn(),
  };
});

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
    id: 9,
    lifecycle_status: "published",
    organizations: [],
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
    vi.mocked(readStoredSessionToken).mockReturnValue("teacher-token");
    vi.mocked(fetchCurrentSession).mockResolvedValue(session());
    vi.mocked(fetchTeachingCourseStudents).mockResolvedValue(students(false));
    vi.mocked(fetchTeacherRewardCandidates).mockResolvedValue([]);
  });

  it("keeps course shell context when reward review permission is missing", async () => {
    render(<TeacherCourseRewardsRoute courseId="9" />);

    expect((await screen.findAllByText("Reward review requires course reward candidate permission for this course.")).length).toBeGreaterThan(0);
    expect(screen.getAllByText("Rust Safety").length).toBeGreaterThan(0);
    expect(screen.getAllByText("Teacher User").length).toBeGreaterThan(0);
    expect(screen.getByRole("link", { name: "Course workspace" })).toHaveAttribute("href", "/teach/courses/9");
    expect(fetchTeacherRewardCandidates).not.toHaveBeenCalled();
  });

  it("keeps course shell context when the reward API denies direct navigation", async () => {
    vi.mocked(fetchTeachingCourseStudents).mockResolvedValue(students(true));
    vi.mocked(fetchTeacherRewardCandidates).mockRejectedValue(
      new TeacherRequestError("User does not have reward candidate permission", 403, "permission_denied"),
    );

    render(<TeacherCourseRewardsRoute courseId="9" />);

    expect((await screen.findAllByText("User does not have reward candidate permission")).length).toBeGreaterThan(0);
    await waitFor(() => expect(fetchTeacherRewardCandidates).toHaveBeenCalled());
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
