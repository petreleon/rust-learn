import { type OrganizationCourseListItem } from "@/lib/organization";
import { type UpdateOrganizationCoursePayload } from "@/lib/organization/UpdateOrganizationCoursePayload";

export type CourseManagementDraft = {
  title: string;
};

export function courseManagementDraft(course: OrganizationCourseListItem): CourseManagementDraft {
  return {
    title: course.title,
  };
}

export function courseManagementPayload(draft: CourseManagementDraft): UpdateOrganizationCoursePayload {
  return {
    title: draft.title.trim(),
  };
}

export function courseManagementValidation(draft: CourseManagementDraft): string | null {
  return draft.title.trim() ? null : "Course title is required.";
}

export function canManageOrganizationCourse(course: OrganizationCourseListItem | null) {
  return Boolean(course?.permissions.can_manage_course_settings);
}
