import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { organizationJsonRequest } from "./organizationJsonRequest";
import { type OrganizationWalletLinkResult } from "./OrganizationWalletLinkResult";
import { type OrganizationWalletOptions } from "./OrganizationWalletOptions";

export async function linkOrganizationWallet({
  apiRoot = "/api",
  organizationId,
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: OrganizationWalletOptions): Promise<OrganizationWalletLinkResult> {
  return organizationJsonRequest({
    apiRoot,
    path: `/wallets/organizations/${organizationId}/link`,
    method: "POST",
    timeoutMs,
    token,
  });
}
