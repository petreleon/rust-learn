import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { organizationJsonRequest } from "./organizationJsonRequest";
import { type OrganizationDashboardOptions } from "./OrganizationDashboardOptions";
import { type OrganizationDashboardSummary } from "./OrganizationDashboardSummary";

export async function fetchOrganizationDashboard({
  apiRoot = "/api",
  organizationId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: OrganizationDashboardOptions): Promise<OrganizationDashboardSummary> {
  return organizationJsonRequest({
    apiRoot,
    path: `/organizations/${organizationId}/dashboard`,
    timeoutMs,
    token,
  });
}
