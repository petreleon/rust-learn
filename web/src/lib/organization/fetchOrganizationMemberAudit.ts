import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { organizationJsonRequest } from "./organizationJsonRequest";
import { type OrganizationMemberAuditEvent } from "./OrganizationMemberAuditEvent";
import { type OrganizationRequestOptions } from "./OrganizationRequestOptions";

export async function fetchOrganizationMemberAudit({
  apiRoot = "/api",
  organizationId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
  userId,
}: OrganizationRequestOptions & { organizationId: number; userId: number }): Promise<OrganizationMemberAuditEvent[]> {
  return organizationJsonRequest({
    apiRoot,
    path: `/organizations/${organizationId}/members/${userId}/audit`,
    timeoutMs,
    token,
  });
}
