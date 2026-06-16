import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type TeacherApplication } from "@/lib/teacher/TeacherApplication";
import { type TeacherApplicationSnapshot } from "@/lib/teacher/TeacherApplicationSnapshot";
import { type TeacherCourseDashboardItem } from "@/lib/teacher/TeacherCourseDashboardItem";
import { type TeacherCoursesResponse } from "@/lib/teacher/TeacherCoursesResponse";

const emptyScope = {
  capabilities: [],
  delegated_permissions: [],
  direct_permissions: [],
  effective_permissions: [],
  roles: [],
};

export function teachingSession(overrides: Partial<CurrentSession> = {}): CurrentSession {
  return {
    access: {
      learner: true,
      organization: false,
      platform_admin: false,
      teacher: true,
      teacher_application: true,
    },
    courses: [
      {
        ...emptyScope,
        capabilities: [{ enabled: true, key: "teaching", label: "Teaching", permissions: [] }],
        id: 9,
        lifecycle_status: "published",
        title: "Rust Safety",
      },
    ],
    delegated_permissions: [],
    organizations: [],
    platform: emptyScope,
    user: {
      email: "teacher@example.com",
      email_verified: true,
      id: 2,
      kyc_verified: true,
      name: "Teacher User",
    },
    ...overrides,
  };
}

export function teachingCourses(
  courses: TeacherCourseDashboardItem[] = [teachingCourse()],
): TeacherCoursesResponse {
  return {
    courses,
    lifecycle_status: null,
    limit: 20,
    offset: 0,
    search: null,
    total: courses.length,
  };
}

export function teachingCourse(
  overrides: Partial<TeacherCourseDashboardItem> = {},
): TeacherCourseDashboardItem {
  return {
    content: { chapter_count: 1, content_count: 2, content_types: ["article"], has_content: true },
    description: null,
    id: 9,
    lifecycle_status: "published",
    organizations: [{ id: 3, name: "Rust Org" }],
    prerequisites: null,
    permissions: {
      can_approve_reward_candidates: true,
      can_manage_content: true,
      can_manage_enrollments: true,
      can_manage_reward_rules: false,
      can_manage_settings: true,
      can_view_reward_candidates: true,
    },
    reward_queue: {
      failed_count: 0,
      pending_teacher_count: 1,
      teacher_approved_count: 0,
    },
    rewards: {
      active_policy_count: 1,
      available: true,
      event_types: ["lesson_completed"],
      payment_strategies: ["learn_token"],
      token_amounts: ["10"],
    },
    roster: {
      enrolled_student_count: 12,
      pending_join_request_count: 1,
      waitlisted_join_request_count: 1,
    },
    title: "Rust Safety",
    topics: null,
    ...overrides,
  };
}

export function applicationSnapshot(
  application: TeacherApplication | null = teacherApplication(),
): TeacherApplicationSnapshot {
  return {
    application,
    audit_events: [],
  };
}

function teacherApplication(): TeacherApplication {
  return {
    applicant_user_id: 2,
    created_at: "2026-06-12T10:00:00Z",
    decided_at: "2026-06-12T11:00:00Z",
    decision_reason: null,
    experience_summary: "Teaching Rust teams.",
    id: 44,
    idempotency_key: null,
    organization_sponsor_id: 3,
    portfolio_links: ["https://example.com/teacher"],
    requested_course_id: 9,
    requested_organization_id: null,
    requested_scope: "course",
    reviewer_id: 1,
    status: "approved",
    updated_at: "2026-06-12T11:00:00Z",
  };
}
