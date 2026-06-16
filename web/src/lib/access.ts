import {
  type CurrentSession,
  type CourseSessionScope,
  type OrganizationSessionScope,
  type PlatformSessionScope,
} from "@/lib/session";

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

  if (session.access) {
    return {
      learner: session.access.learner,
      teacher: session.access.teacher,
      organization: session.access.organization,
      platformAdmin: session.access.platform_admin,
    };
  }

  return {
    learner: true,
    teacher: hasLegacyTeacherAccess(session),
    organization: session.organizations.some(hasOrganizationAccess),
    platformAdmin: hasLegacyPlatformAdminAccess(session),
  };
}

export function hasAnyScopePermission(scope: PlatformSessionScope, permissions: Set<string>) {
  return scope.effective_permissions.some((permission) => permissions.has(permission));
}

export function hasTeacherApplicationAccess(session: CurrentSession) {
  return session.access.teacher_application;
}

export function hasTeacherCourseAccess(course: CourseSessionScope) {
  return (course.capabilities ?? []).some((capability) => capability.key === "teaching" && capability.enabled);
}

export function hasOrganizationAccess(organization: OrganizationSessionScope) {
  return (
    organization.roles.length > 0 ||
    organization.direct_permissions.length > 0 ||
    organization.delegated_permissions.length > 0 ||
    (organization.capabilities ?? []).some((capability) => capability.enabled)
  );
}

export function hasPlatformAdminAccess(session: CurrentSession) {
  return session.access.platform_admin;
}

export function countDelegatedPermissions(session?: CurrentSession | null) {
  return session?.delegated_permissions.length || 0;
}

function hasLegacyTeacherAccess(session: CurrentSession) {
  return (
    session.platform.effective_permissions.includes("SUBMIT_TEACHER_APPLICATION") ||
    session.courses.some(hasTeacherCourseAccess)
  );
}

function hasLegacyPlatformAdminAccess(session: CurrentSession) {
  return session.platform.effective_permissions.some((permission) => permission !== "SUBMIT_TEACHER_APPLICATION");
}
