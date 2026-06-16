import { type OrganizationWorkspaceItem } from "@/lib/organization";

const createCoursePermission = "CREATE_COURSE";

export function canCreateOrganizationCourse(organization: OrganizationWorkspaceItem | null) {
  return Boolean(organization?.effectivePermissions.includes(createCoursePermission));
}

export function validateOrganizationCourseTitle(title: string): string | null {
  return title.trim() ? null : "Course title is required.";
}
