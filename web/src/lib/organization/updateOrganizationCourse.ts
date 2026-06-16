import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { organizationJsonRequest } from "./organizationJsonRequest";
import { type OrganizationCourseResponse } from "./OrganizationCourseResponse";
import { type UpdateOrganizationCourseOptions } from "./UpdateOrganizationCourseOptions";

export function updateOrganizationCourse({
  apiRoot = "/api",
  courseId,
  payload,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: UpdateOrganizationCourseOptions): Promise<OrganizationCourseResponse> {
  return organizationJsonRequest({
    apiRoot,
    body: JSON.stringify(payload),
    method: "PUT",
    path: `/courses/${courseId}`,
    timeoutMs,
    token,
  });
}
