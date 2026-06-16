import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type TeacherAssessment, type TeacherCourseWorkspaceContent, type TeacherCourseWorkspaceResponse } from "@/lib/teacher";

export function courseContentSession(): CurrentSession {
  return {
    access: { learner: true, organization: false, platform_admin: false, teacher: true, teacher_application: false },
    courses: [],
    delegated_permissions: [],
    organizations: [],
    platform: { capabilities: [], delegated_permissions: [], direct_permissions: [], effective_permissions: [], roles: [] },
    user: { email: "teacher@example.com", email_verified: true, id: 7, kyc_verified: true, name: "Teacher User" },
  };
}

export function textContent(): TeacherCourseWorkspaceContent {
  return {
    content_type: "article",
    data: "Original lesson",
    data_present: true,
    display_state: "ready",
    id: 11,
    order: 0,
    processing_error: null,
    processing_status: null,
    publication_status: "inherits_course_published",
  };
}

export function failedVideoContent(): TeacherCourseWorkspaceContent {
  return {
    content_type: "video/mp4",
    data: "courses/9/intro.mp4",
    data_present: true,
    display_state: "failed_processing",
    id: 12,
    order: 1,
    processing_error: "Transcode failed",
    processing_status: "failed",
    publication_status: "inherits_course_published",
  };
}

export function courseContentAssessment({
  published = false,
  title = "Draft quiz",
}: {
  published?: boolean;
  title?: string;
} = {}): TeacherAssessment {
  return {
    course_id: 9,
    created_at: "2026-06-16T08:00:00Z",
    description: "Ownership check",
    id: 31,
    max_attempts: 3,
    passing_score: 70,
    published,
    questions: [{
      assessment_id: 31,
      correct_answer: "Ownership",
      id: 41,
      options: ["Ownership", "Prototype chains"],
      order: 0,
      points: 2,
      question_type: "multiple_choice",
      text: "Which concept prevents aliasing bugs?",
    }],
    title,
    updated_at: "2026-06-16T08:00:00Z",
  };
}

export function courseContentWorkspace(contents: TeacherCourseWorkspaceContent[] = [textContent()]): TeacherCourseWorkspaceResponse {
  return {
    chapters: [{ contents, id: 3, order: 0, title: "Intro" }],
    course: {
      content: { chapter_count: 1, content_count: contents.length, content_types: ["article"], has_content: contents.length > 0 },
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
      rewards: { active_policy_count: 0, available: false, event_types: [], payment_strategies: [], token_amounts: [] },
      roster: { enrolled_student_count: 0, pending_join_request_count: 0, waitlisted_join_request_count: 0 },
      title: "Rust Safety",
    },
    publication: { content_publication_status_supported: false, course_lifecycle_status: "published" },
    teacher_roles: ["TEACHER"],
  };
}
