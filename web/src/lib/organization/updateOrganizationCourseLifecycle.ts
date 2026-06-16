import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { organizationJsonRequest } from "./organizationJsonRequest";
import { type OrganizationCourseResponse } from "./OrganizationCourseResponse";
import { type UpdateOrganizationCourseLifecycleOptions } from "./UpdateOrganizationCourseLifecycleOptions";

export function updateOrganizationCourseLifecycle({
  apiRoot = "/api",
  courseId,
  status,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: UpdateOrganizationCourseLifecycleOptions): Promise<OrganizationCourseResponse> {
  return organizationJsonRequest({
    apiRoot,
    body: JSON.stringify({ status }),
    method: "PUT",
    path: `/courses/${courseId}/lifecycle`,
    timeoutMs,
    token,
  });
}
