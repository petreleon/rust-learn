import { type PlatformTeacherApplicationItem } from "@/lib/admin/PlatformTeacherApplicationItem";
import { type PlatformTeacherApplicationsResponse } from "@/lib/admin/PlatformTeacherApplicationsResponse";
import { type TeacherApplication } from "@/lib/admin/TeacherApplication";
import { type TeacherApplicationAuditEvent } from "@/lib/admin/TeacherApplicationAuditEvent";
import { type CurrentSession } from "@/lib/session/CurrentSession";

const teacherApplicationPermissions = [
  "REVIEW_TEACHER_APPLICATIONS",
  "APPROVE_TEACHER_APPLICATION",
  "REJECT_TEACHER_APPLICATION",
];

export function adminTeacherApplicationSession(permissions: string[]): CurrentSession {
  return {
    access: {
      learner: true,
      organization: false,
      platform_admin: permissions.length > 0,
      teacher: false,
      teacher_application: false,
    },
    courses: [],
    delegated_permissions: [],
    organizations: [],
    platform: {
      capabilities: [
        {
          enabled: teacherApplicationPermissions.some((permission) => permissions.includes(permission)),
          key: "teacher_applications",
          label: "Teacher application review",
          permissions: teacherApplicationPermissions,
        },
      ],
      delegated_permissions: [],
      direct_permissions: permissions,
      effective_permissions: permissions,
      roles: permissions.length ? ["platform_admin"] : [],
    },
    user: {
      email: "admin@example.com",
      email_verified: true,
      id: 1,
      kyc_verified: true,
      name: "Admin User",
    },
  };
}

export function teacherApplication(
  overrides: Partial<PlatformTeacherApplicationItem> = {},
): PlatformTeacherApplicationItem {
  return {
    applicant: { email: "ada.teacher@example.com", id: 22, name: "Ada Teacher" },
    audit: { latest_event_at: "2026-06-12T10:00:00Z", total_events: 1 },
    created_at: "2026-06-12T10:00:00Z",
    decided_at: null,
    decision_reason: null,
    experience_summary: "Five years teaching Rust teams.",
    id: 41,
    portfolio_links: ["https://example.com/ada"],
    requested_course: { id: 4, title: "Rust Foundations" },
    requested_organization: null,
    requested_scope: "course",
    reviewer: null,
    sponsor_organization: { id: 2, name: "Rust Org" },
    status: "submitted",
    updated_at: "2026-06-12T10:00:00Z",
    ...overrides,
  };
}

export function teacherApplicationResponse(
  applications: PlatformTeacherApplicationItem[],
): PlatformTeacherApplicationsResponse {
  return {
    applications,
    limit: 8,
    offset: 0,
    operator_permissions: {
      can_approve_applications: true,
      can_reject_applications: true,
      can_request_changes: true,
      can_view_applications: true,
    },
    search: null,
    status: "submitted",
    summary: {
      approved: applications.filter((item) => item.status === "approved").length,
      needs_changes: applications.filter((item) => item.status === "needs_changes").length,
      rejected: applications.filter((item) => item.status === "rejected").length,
      submitted: applications.filter((item) => item.status === "submitted").length,
      total: applications.length,
    },
    total: applications.length,
  };
}

export function teacherApplicationDecisionResult(
  overrides: Partial<TeacherApplication> = {},
): TeacherApplication {
  return {
    applicant_user_id: 22,
    created_at: "2026-06-12T10:00:00Z",
    decided_at: "2026-06-12T11:00:00Z",
    decision_reason: "Ready to teach",
    experience_summary: "Five years teaching Rust teams.",
    id: 41,
    idempotency_key: null,
    organization_sponsor_id: 2,
    portfolio_links: ["https://example.com/ada"],
    requested_course_id: 4,
    requested_organization_id: null,
    requested_scope: "course",
    reviewer_id: 1,
    status: "approved",
    updated_at: "2026-06-12T11:00:00Z",
    ...overrides,
  };
}

export function teacherApplicationAuditEvent(
  overrides: Partial<TeacherApplicationAuditEvent> = {},
): TeacherApplicationAuditEvent {
  return {
    actor_user_id: 22,
    application_id: 41,
    created_at: "2026-06-12T10:00:00Z",
    event_type: "submitted",
    from_status: null,
    id: 15,
    reason: "Initial submission",
    to_status: "submitted",
    ...overrides,
  };
}
