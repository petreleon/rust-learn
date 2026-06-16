import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type TeacherCourseWorkspaceResponse } from "@/lib/teacher/TeacherCourseWorkspaceResponse";
import { type CourseCompletionTerms } from "./CourseCompletionTerms";

export function openTerms(terms: CourseCompletionTerms[]) {
  return terms.find((item) => item.status === "submitted" || item.status === "countered") ?? null;
}

export function statusLabel(status: string) {
  return status
    .split("_")
    .filter(Boolean)
    .map((part) => part[0]?.toUpperCase() + part.slice(1))
    .join(" ");
}

export function formatTermsDate(value: string | null) {
  if (!value) return "Not recorded";
  return new Intl.DateTimeFormat(undefined, {
    dateStyle: "medium",
    timeStyle: "short",
  }).format(new Date(value));
}

export function canProposeTerms(workspace: TeacherCourseWorkspaceResponse) {
  return (
    workspace.course.permissions.can_manage_settings ||
    workspace.course.permissions.can_manage_reward_rules
  );
}

export function canDecideTerms(
  session: CurrentSession | null,
  workspace: TeacherCourseWorkspaceResponse,
) {
  if (!session) return false;
  if (hasPermission(session.platform.effective_permissions)) return true;
  const courseOrganizationIds = new Set(workspace.course.organizations.map(({ id }) => id));
  return session.organizations.some(
    (organization) =>
      courseOrganizationIds.has(organization.id) &&
      hasPermission(organization.effective_permissions),
  );
}

function hasPermission(permissions: string[]) {
  return (
    permissions.includes("SET_REWARD_POLICY") ||
    permissions.includes("MANAGE_ORG_REWARD_BUDGET")
  );
}
