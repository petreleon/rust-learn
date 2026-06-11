import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { organizationJsonRequest } from "./organizationJsonRequest";
import { type OrganizationWalletAudit } from "./OrganizationWalletAudit";
import { type OrganizationWalletOptions } from "./OrganizationWalletOptions";

export async function fetchOrganizationWalletAudit({
  apiRoot = "/api",
  organizationId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: OrganizationWalletOptions): Promise<OrganizationWalletAudit> {
  return organizationJsonRequest({
    apiRoot,
    path: `/wallets/organizations/${organizationId}/audit`,
    timeoutMs,
    token,
  });
}
