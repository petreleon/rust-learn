import { DEFAULT_TIMEOUT_MS } from "./DEFAULT_TIMEOUT_MS";
import { learnerErrorFromResponse } from "./learnerErrorFromResponse";
import { learnerRawRequest } from "./learnerRawRequest";
import { type LearnerRequestOptions } from "./LearnerRequestOptions";
import { type WalletAudit } from "./WalletAudit";

export async function fetchMyWalletAudit({
  apiRoot = "/api",
  timeoutMs = DEFAULT_TIMEOUT_MS,
  token,
}: LearnerRequestOptions): Promise<WalletAudit | null> {
  const response = await learnerRawRequest({
    method: "GET",
    timeoutMs,
    token,
    url: `${apiRoot}/wallets/me/audit`,
  });

  if (response.status === 404) {
    return null;
  }

  if (!response.ok) {
    throw await learnerErrorFromResponse(response, "Wallet history could not be loaded.");
  }

  return (await response.json()) as WalletAudit;
}
