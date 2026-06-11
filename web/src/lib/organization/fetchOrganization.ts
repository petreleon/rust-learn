import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { organizationJsonRequest } from "./organizationJsonRequest";
import { type OrganizationDetail } from "./OrganizationDetail";
import { type OrganizationRequestOptions } from "./OrganizationRequestOptions";

export async function fetchOrganization({
  apiRoot = "/api",
  organizationId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: OrganizationRequestOptions & { organizationId: number }): Promise<OrganizationDetail> {
  return organizationJsonRequest({
    apiRoot,
    path: `/organizations/${organizationId}`,
    timeoutMs,
    token,
  });
}
