import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type OrganizationCourseList } from "@/lib/organization/OrganizationCourseList";

const emptyScope = {
  capabilities: [],
  delegated_permissions: [],
  direct_permissions: [],
  effective_permissions: [],
  roles: [],
};

export function sessionFixture(): CurrentSession {
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

export function courseListFixture(): OrganizationCourseList {
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
      rewards: {
        active_policy_count: 1,
        available: true,
        event_types: ["manual_completion"],
        payment_strategies: ["mint"],
        token_amounts: ["25"],
      },
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
