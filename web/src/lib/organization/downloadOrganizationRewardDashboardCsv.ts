import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { filenameFromContentDisposition } from "./filenameFromContentDisposition";
import { organizationRawRequest } from "./organizationRawRequest";
import { type OrganizationCsvDownload } from "./OrganizationCsvDownload";
import { type OrganizationReportOptions } from "./OrganizationReportOptions";

export async function downloadOrganizationRewardDashboardCsv({
  apiRoot = "/api",
  from,
  organizationId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  to,
  token,
}: OrganizationReportOptions & { from?: string; to?: string }): Promise<OrganizationCsvDownload> {
  const query = new URLSearchParams();
  if (from) query.set("from", from);
  if (to) query.set("to", to);
  const suffix = query.toString();
  const response = await organizationRawRequest({
    apiRoot,
    path: `/reports/organizations/${organizationId}/reward-dashboard.csv${suffix ? `?${suffix}` : ""}`,
    timeoutMs,
    token,
    accept: "text/csv, text/plain",
  });
  const body = await response.text();

  return {
    body,
    filename: filenameFromContentDisposition(response.headers.get("content-disposition")) ||
      `organization-${organizationId}-reward-dashboard.csv`,
  };
}
