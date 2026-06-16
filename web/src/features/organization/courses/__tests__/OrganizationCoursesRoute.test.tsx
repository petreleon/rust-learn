import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vi } from "vitest";
import { type CurrentSession } from "@/lib/session/CurrentSession";
import { readStoredSessionToken } from "@/lib/session";
import { type OrganizationCourseList } from "@/lib/organization/OrganizationCourseList";
import { useOrganizationSession } from "@/features/organization/shared/route-kit/useOrganizationSession";
import { OrganizationCoursesRoute } from "../route/OrganizationCoursesRoute";
import { createDraftOrganizationCourse, loadOrganizationCourses } from "../api/courseApi";

vi.mock("next/navigation", () => ({
  usePathname: () => "/organizations/4/courses",
  useRouter: () => ({ push: vi.fn() }),
}));

vi.mock("@/lib/session", async () => {
  const actual = await vi.importActual<typeof import("@/lib/session")>("@/lib/session");
  return {
    ...actual,
    clearStoredSessionToken: vi.fn(),
    readStoredSessionToken: vi.fn(),
  };
});

vi.mock("@/features/organization/shared/route-kit/useOrganizationSession", () => ({
  useOrganizationSession: vi.fn(),
}));

vi.mock("../api/courseApi", () => ({
  createDraftOrganizationCourse: vi.fn(),
  loadOrganizationCourses: vi.fn(),
}));

const emptyScope = { capabilities: [], delegated_permissions: [], direct_permissions: [], effective_permissions: [], roles: [] };

describe("OrganizationCoursesRoute", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(readStoredSessionToken).mockReturnValue("org-token");
    vi.mocked(useOrganizationSession).mockReturnValue({
      error: null,
      hasToken: true,
      loadSession: vi.fn(),
      loadState: "success",
      session: sessionFixture(),
      signOut: vi.fn(),
    });
    vi.mocked(loadOrganizationCourses).mockResolvedValue(courseListFixture());
    vi.mocked(createDraftOrganizationCourse).mockResolvedValue({
      description: null,
      id: 41,
      lifecycle_status: "draft",
      prerequisites: null,
      title: "Org Rust Lab",
      topics: null,
    });
  });

  it("loads organization courses through the feature API", async () => {
    render(<OrganizationCoursesRoute organizationId="4" />);

    expect(await screen.findByRole("heading", { name: "Course list" })).toBeVisible();
    expect(screen.getByText("Rust Ownership Lab")).toBeVisible();
    expect(loadOrganizationCourses).toHaveBeenCalledWith({
      lifecycleStatus: "",
      limit: 6,
      offset: 0,
      organizationId: 4,
      rewardAvailable: null,
      search: "",
      token: "org-token",
    });
  });

  it("creates a draft organization course through the feature API", async () => {
    const user = userEvent.setup();
    render(<OrganizationCoursesRoute organizationId="4" />);

    await user.type(await screen.findByPlaceholderText("Ferris Org course title"), "Org Rust Lab");
    await user.click(screen.getByRole("button", { name: "Create draft course" }));

    await waitFor(() =>
      expect(createDraftOrganizationCourse).toHaveBeenCalledWith({
        organizationId: 4,
        title: "Org Rust Lab",
        token: "org-token",
      }),
    );
    expect(await screen.findByText("Org Rust Lab was created as draft.")).toBeVisible();
    expect(loadOrganizationCourses).toHaveBeenCalledTimes(2);
  });
});

function sessionFixture(): CurrentSession {
  return {
    access: { learner: true, organization: true, platform_admin: false, teacher: false, teacher_application: false },
    courses: [],
    delegated_permissions: [],
    organizations: [{
      ...emptyScope,
      capabilities: [{ enabled: true, key: "courses", label: "Courses", permissions: ["VIEW_ORGANIZATION"] }],
      direct_permissions: ["VIEW_ORGANIZATION", "CREATE_COURSE"],
      effective_permissions: ["VIEW_ORGANIZATION", "CREATE_COURSE"],
      id: 4,
      name: "Ferris Org",
      roles: ["ORG_ADMIN"],
    }],
    platform: emptyScope,
    user: { email: "operator@example.com", email_verified: true, id: 2, kyc_verified: true, name: "Org Operator" },
  };
}

function courseListFixture(): OrganizationCourseList {
  return {
    courses: [{
      content: { chapter_count: 2, content_count: 4, content_types: ["article"], has_content: true },
      id: 9,
      lifecycle_status: "published",
      permissions: {
        can_create_courses: true,
        can_manage_course_settings: true,
        can_manage_enrollments: true,
        can_manage_reward_budget: true,
        can_submit_reward_events: true,
        can_view_courses: true,
        can_view_reward_reports: true,
      },
      reward_queue: { failed_count: 0, pending_teacher_count: 1, teacher_approved_count: 0 },
      rewards: { active_policy_count: 1, available: true, event_types: ["manual_completion"], payment_strategies: ["mint"], token_amounts: ["25"] },
      roster: { enrolled_student_count: 8, pending_join_request_count: 2, waitlisted_join_request_count: 0 },
      teachers: [{ id: 7, name: "Ada Teacher" }],
      title: "Rust Ownership Lab",
    }],
    lifecycle_status: null,
    limit: 8,
    offset: 0,
    organization: { id: 4, name: "Ferris Org" },
    reward_available: null,
    search: null,
    total: 1,
  };
}
