import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { adminJsonRequest } from "./adminJsonRequest";
import { type AdminRequestOptions } from "./AdminRequestOptions";
import { type PlatformWalletReconciliation } from "./PlatformWalletReconciliation";

export async function fetchPlatformWalletReconciliation({
  apiRoot = "/api",
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: AdminRequestOptions): Promise<PlatformWalletReconciliation> {
  return adminJsonRequest({
    apiRoot,
    path: "/reports/platform/wallet-reconciliation",
    timeoutMs,
    token,
  });
}
