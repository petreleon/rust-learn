import {
  type CurrentSession,
  type CourseSessionScope,
  type OrganizationSessionScope,
  type PlatformSessionScope,
} from "@/lib/session";

const teacherPermissions = new Set([
  "APPROVE_COURSE_CONTENT",
  "APPROVE_COURSE_JOIN_REQUESTS",
  "APPROVE_STUDENT_REWARD_CANDIDATE",
  "MANAGE_ASSESSMENT_TEMPLATES",
  "MANAGE_COURSE_ENROLLMENTS",
  "MANAGE_COURSE_REWARD_RULES",
  "MANAGE_COURSE_SETTINGS",
  "SUBMIT_COURSE_REWARD_EVENT",
]);

const teacherApplicationPermissions = new Set([
  "SUBMIT_TEACHER_APPLICATION",
  "VIEW_ORG_TEACHER_APPLICATIONS",
]);

const organizationPermissions = new Set([
  "ASSIGN_ROLES_TO_ORG_USERS",
  "INVITE_USER_TO_ORGANIZATION",
  "MANAGE_ORG_BILLING",
  "MANAGE_ORG_MEMBERS",
  "MANAGE_ORG_REWARD_BUDGET",
  "MANAGE_ORG_SETTINGS",
  "MANAGE_ORG_WALLETS",
  "NOMINATE_TEACHER_FOR_PLATFORM_REVIEW",
  "SUBMIT_ORG_COURSE_REWARD_EVENT",
  "VIEW_ORGANIZATION",
  "VIEW_ORG_REWARD_REPORTS",
  "VIEW_ORG_TEACHER_APPLICATIONS",
]);

const platformAdminPermissions = new Set([
  "APPROVE_CENTRALIZED_TRANSFER",
  "APPROVE_REWARD_AMOUNT",
  "APPROVE_TEACHER_APPLICATION",
  "BLOCK_REWARD_ORGANIZATION",
  "BLOCK_REWARD_TEACHER",
  "EXPORT_DATA",
  "MANAGE_API_KEYS",
  "MANAGE_BILLING",
  "MANAGE_ETHEREUM_ACCOUNTS",
  "MANAGE_EXPORT_JOBS",
  "MANAGE_INTEGRATIONS",
  "MANAGE_PLATFORM_SETTINGS",
  "MANAGE_REWARD_FRAUD_BLOCKS",
  "MANAGE_ROLE_PERMISSIONS",
  "MANAGE_S3_OBJECTS",
  "MANAGE_SMART_CONTRACTS",
  "MANAGE_TEST_SUITES",
  "MANAGE_WALLETS",
  "REJECT_TEACHER_APPLICATION",
  "REVIEW_TEACHER_APPLICATIONS",
  "VIEW_ANALYTICS_DASHBOARD",
  "VIEW_AUDIT_LOGS",
  "VIEW_FINANCIAL_REPORTS",
  "VIEW_REPORT",
  "VIEW_REWARD_AUDIT",
  "VIEW_ROLE_ASSIGNMENTS",
  "VIEW_SENSITIVE_TRANSACTIONS",
  "VIEW_TRANSACTIONS",
]);

export type AccessSummary = {
  learner: boolean;
  teacher: boolean;
  organization: boolean;
  platformAdmin: boolean;
};

export function accessSummary(session?: CurrentSession | null): AccessSummary {
  if (!session) {
    return {
      learner: false,
      teacher: false,
      organization: false,
      platformAdmin: false,
    };
  }

  return {
    learner: true,
    teacher:
      hasAnyScopePermission(session.platform, teacherApplicationPermissions) ||
      session.courses.some(hasTeacherCourseAccess),
    organization: session.organizations.some(hasOrganizationAccess),
    platformAdmin: hasAnyScopePermission(session.platform, platformAdminPermissions),
  };
}

export function hasAnyScopePermission(scope: PlatformSessionScope, permissions: Set<string>) {
  return scope.effective_permissions.some((permission) => permissions.has(permission));
}

export function hasTeacherApplicationAccess(session: CurrentSession) {
  return hasAnyScopePermission(session.platform, teacherApplicationPermissions);
}

export function hasTeacherCourseAccess(course: CourseSessionScope) {
  return hasAnyScopePermission(course, teacherPermissions);
}

export function hasOrganizationAccess(organization: OrganizationSessionScope) {
  return hasAnyScopePermission(organization, organizationPermissions);
}

export function hasPlatformAdminAccess(session: CurrentSession) {
  return hasAnyScopePermission(session.platform, platformAdminPermissions);
}

export function countDelegatedPermissions(session?: CurrentSession | null) {
  return session?.delegated_permissions.length || 0;
}
