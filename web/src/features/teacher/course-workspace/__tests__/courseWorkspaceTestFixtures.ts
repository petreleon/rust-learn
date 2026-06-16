import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type TeacherCourseDashboardItem } from "@/lib/teacher/TeacherCourseDashboardItem";
import { type TeacherCourseWorkspaceResponse } from "@/lib/teacher/TeacherCourseWorkspaceResponse";

const emptyScope = {
  capabilities: [],
  delegated_permissions: [],
  direct_permissions: [],
  effective_permissions: [],
  roles: [],
};

export function courseWorkspaceSession(): CurrentSession {
  return {
    access: {
      learner: true,
      organization: false,
      platform_admin: false,
      teacher: true,
      teacher_application: false,
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
  };
}

export function courseWorkspace(): TeacherCourseWorkspaceResponse {
  return {
    chapters: [
      {
        contents: [
          {
            content_type: "article",
            data: "Borrowing rules",
            data_present: true,
            display_state: "ready",
            id: 101,
            order: 1,
            processing_error: null,
            processing_status: null,
            publication_status: "published",
          },
        ],
        id: 15,
        order: 1,
        title: "Ownership",
      },
    ],
    course: workspaceCourse(),
    publication: {
      content_publication_status_supported: false,
      course_lifecycle_status: "published",
    },
    teacher_roles: ["TEACHER"],
  };
}

function workspaceCourse(): TeacherCourseDashboardItem {
  return {
    content: { chapter_count: 1, content_count: 1, content_types: ["article"], has_content: true },
    id: 9,
    lifecycle_status: "published",
    organizations: [{ id: 3, name: "Rust Org" }],
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
      waitlisted_join_request_count: 0,
    },
    title: "Rust Safety",
  };
}
