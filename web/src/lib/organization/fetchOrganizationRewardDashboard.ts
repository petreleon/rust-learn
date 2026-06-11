import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { organizationJsonRequest } from "./organizationJsonRequest";
import { type OrganizationReportOptions } from "./OrganizationReportOptions";
import { type OrganizationRewardDashboard } from "./OrganizationRewardDashboard";

export async function fetchOrganizationRewardDashboard({
  apiRoot = "/api",
  from,
  organizationId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  to,
  token,
}: OrganizationReportOptions & { from?: string; to?: string }): Promise<OrganizationRewardDashboard> {
  const query = new URLSearchParams();
  if (from) query.set("from", from);
  if (to) query.set("to", to);
  const suffix = query.toString();
  return organizationJsonRequest({
    apiRoot,
    path: `/reports/organizations/${organizationId}/reward-dashboard${suffix ? `?${suffix}` : ""}`,
    timeoutMs,
    token,
  });
}
