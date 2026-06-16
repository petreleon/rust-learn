import {
  createOrganizationCourse,
  fetchOrganizationCourses,
  type OrganizationCourseListOptions,
  updateOrganizationCourse,
  updateOrganizationCourseLifecycle,
} from "@/lib/organization";
import {
  courseManagementPayload,
  type CourseManagementDraft,
} from "../model/courseManagementModel";

export function loadOrganizationCourses(options: OrganizationCourseListOptions) {
  return fetchOrganizationCourses(options);
}

export function createDraftOrganizationCourse({
  organizationId,
  title,
  token,
}: {
  organizationId: number;
  title: string;
  token: string;
}) {
  return createOrganizationCourse({
    payload: { organization_ids: [organizationId], title: title.trim() },
    token,
  });
}

export function saveOrganizationCourseSettings({
  courseId,
  draft,
  token,
}: {
  courseId: number;
  draft: CourseManagementDraft;
  token: string;
}) {
  return updateOrganizationCourse({
    courseId,
    payload: courseManagementPayload(draft),
    token,
  });
}

export function saveOrganizationCourseLifecycle({
  courseId,
  status,
  token,
}: {
  courseId: number;
  status: string;
  token: string;
}) {
  return updateOrganizationCourseLifecycle({ courseId, status, token });
}
