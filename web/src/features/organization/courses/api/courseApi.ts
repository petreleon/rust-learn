import {
  createOrganizationCourse,
  fetchOrganizationCourses,
  type OrganizationCourseListOptions,
} from "@/lib/organization";

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
