import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { organizationJsonRequest } from "./organizationJsonRequest";
import { type CreateOrganizationCourseOptions } from "./CreateOrganizationCourseOptions";
import { type OrganizationCourseResponse } from "./OrganizationCourseResponse";

export async function createOrganizationCourse({
  apiRoot = "/api",
  payload,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: CreateOrganizationCourseOptions): Promise<OrganizationCourseResponse> {
  return organizationJsonRequest<OrganizationCourseResponse>({
    apiRoot,
    body: JSON.stringify(payload),
    method: "POST",
    path: "/courses",
    timeoutMs,
    token,
  });
}
