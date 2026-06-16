import { type CurrentSession } from "@/lib/session/CurrentSession";
import { type CreateTeacherCoursePayload } from "@/lib/teacher/CreateTeacherCoursePayload";

const createCoursePermission = "CREATE_COURSE";

export type CourseCreationDraft = {
  targetValue: string;
  title: string;
};

export type CourseCreationTarget = {
  label: string;
  organizationIds: number[];
  value: string;
};

export function courseCreationTargets(session: CurrentSession): CourseCreationTarget[] {
  const targets: CourseCreationTarget[] = [];
  if (hasPermission(session.platform.effective_permissions)) {
    targets.push({ label: "Personal or platform course", organizationIds: [], value: "platform" });
  }
  for (const organization of session.organizations) {
    if (hasPermission(organization.effective_permissions)) {
      targets.push({
        label: organization.name,
        organizationIds: [organization.id],
        value: `organization:${organization.id}`,
      });
    }
  }
  return targets;
}

export function defaultCourseCreationDraft(targets: CourseCreationTarget[]): CourseCreationDraft {
  return { targetValue: targets[0]?.value || "", title: "" };
}

export function courseCreationPayload(draft: CourseCreationDraft): CreateTeacherCoursePayload {
  return {
    organization_ids: organizationIdsFromTarget(draft.targetValue),
    title: draft.title.trim(),
  };
}

export function courseCreationValidation(draft: CourseCreationDraft): string | null {
  if (!draft.title.trim()) return "Course title is required.";
  if (!draft.targetValue) return "A course owner target is required.";
  return null;
}

function hasPermission(permissions: string[]): boolean {
  return permissions.includes(createCoursePermission);
}

function organizationIdsFromTarget(targetValue: string): number[] {
  if (!targetValue.startsWith("organization:")) return [];
  const organizationId = Number.parseInt(targetValue.replace("organization:", ""), 10);
  return Number.isFinite(organizationId) ? [organizationId] : [];
}
